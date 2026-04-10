use axum::{
    extract::{
	ConnectInfo,
	Path,
	State,
    },
    Form,
    response::{
	Html,
	IntoResponse,
	Redirect,
    },
};
use std::error::Error;
use std::net::SocketAddr;
use sqlx::PgPool;

use crate::{
    INTERNAL_SERVER_ERROR_REPLY,
    USER_SUSPENDED_REPLY,
};
use crate::forms::ReplyForm;
use crate::template::{
    TERA,
};
use crate::helper::{
    Thread,
    ThreadSerializable,
    create_reply,
    hashed,
    number_of_pending_posts_in_last_hour,
};
use crate::moderation::is_user_suspended;

async fn all_threads(
    pool: PgPool,
) -> Result<Vec<ThreadSerializable>, Box<dyn Error>> {
    let q = "\
    SELECT \
    t.id, \
    t.uid, \
    t.subject, \
    t.comment, \
    t.board, \
    t.ctime, \
    t.mtime, \
    t.redacted, \
    COUNT(r.id) AS reply_count \
    FROM thread t \
    LEFT JOIN reply r ON r.tid = t.id \
    GROUP BY t.id \
    ORDER BY t.id desc \
    ";

    let threads: Vec<Thread> = sqlx::query_as::<_, Thread>(q)
	.fetch_all(&pool)
	.await?;

    let serializable_threads: Vec<ThreadSerializable> = threads.into_iter()
        .map(Thread::into_serializable)
        .collect();

    Ok(serializable_threads)
}

async fn thread_with_id(
    id: i32,
    pool: PgPool,
) -> Result<ThreadSerializable, Box<dyn Error>> {
    let q = "\
    SELECT \
    t.id, \
    t.uid, \
    t.subject, \
    t.comment, \
    t.board, \
    t.ctime, \
    t.mtime, \
    t.redacted, \
    COUNT(r.id) AS reply_count \
    FROM thread t \
    LEFT JOIN reply r ON r.tid = t.id \
    WHERE t.id = $1 \
    GROUP BY t.id \
    ";

    let thread: Thread = sqlx::query_as::<_, Thread>(q)
	.bind(&id)
	.fetch_one(&pool)
	.await?;

    let serializable_thread = thread.into_serializable();

    Ok(serializable_thread)
}

async fn thread_with_reply_id(
    reply_id: i32,
    pool: PgPool,
) -> Result<i32, Box<dyn Error>> {
    let q = "\
    SELECT \
    tid \
    FROM reply \
    WHERE id = $1
    ";

    let thread_id: (i32,) = sqlx::query_as::<_, (i32,)>(q)
	.bind(&reply_id)
	.fetch_one(&pool)
	.await?;

    Ok(thread_id.0)
}

pub async fn k_page(
    State(pool): State<PgPool>,
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();
    let threads = all_threads(
	pool,
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("threads", &threads);

    let rendered = TERA.render("k.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("{e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    Ok(Html(content))
}

pub async fn k_thread_page(
    State(pool): State<PgPool>,
    Path(id_hex): Path<String>,
) -> Result<Result<Html<String>, Redirect>, axum::http::StatusCode> {
    let Ok(id_u32) = u32::from_str_radix(&id_hex, 16) else {
	return Err(axum::http::StatusCode::BAD_REQUEST)
    };
    let id = id_u32 as i32;

    let Ok(thread) = thread_with_id(
	id,
	pool.clone(),
    ).await else {
	return if let Ok(tid) = thread_with_reply_id(id, pool).await {
	    let tid_hex = format!("{:03x}", tid);
	    Ok(Err(Redirect::permanent(
		&format!("/k/{}#{}", tid_hex, id_hex
		))))
	} else {
	    Err(axum::http::StatusCode::NOT_FOUND)
	}
    };

    let Ok(replies) = crate::helper::thread_replies(
	id,
	pool,
    ).await else {
	return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    };

    let mut ctx = tera::Context::new();
    ctx.insert("thread", &thread);
    ctx.insert("replies", &replies);

    let rendered = TERA.render("k_thread.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("{e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    Ok(Ok(Html(content)))
}

pub async fn reply_submission(
    state_pool: State<PgPool>,
    Path(tid_hex): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(reply_form): Form<ReplyForm>,
) -> impl IntoResponse {
    let Ok(tid_u32) = u32::from_str_radix(&tid_hex, 16) else {
	return Err(
	    (
		axum::http::StatusCode::BAD_REQUEST,
		"অবৈধ আর্টিকেল আইডি! অনুগ্রহ করে আপনার URL পুনরায় যাচাই করুন।",
	    )
	)
    };
    let tid = tid_u32 as i32;

    let uid = hashed(addr.ip());

    match is_user_suspended(uid, state_pool.clone()).await {
	Ok(true) => return Err(
	    (
		axum::http::StatusCode::FORBIDDEN,
		USER_SUSPENDED_REPLY,
	    )
	),
	Ok(false) => (),
	Err(e) => {
	    eprintln!("Error checking user suspension: {}", e);
	    return Err(
		(
		    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	},
    }

    let number_of_posts_by_user = match number_of_pending_posts_in_last_hour(
	uid,
	state_pool.clone(),
    ).await {
	Ok(number) => number,
	Err(e) => {
	    eprintln!("Error checking user pending posts number: {}", e);
	    return Err(
		(
		    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	},
    };
    if number_of_posts_by_user > 10 {
	return Err(
	    (
		axum::http::StatusCode::TOO_MANY_REQUESTS,
		"আপনার গত এক ঘণ্টায় অনেক অনির্বাচিত পোস্ট রয়েছে। পুনরায় পোস্ট \
		 করার আগে অনুগ্রহ করে কিছুক্ষণ অপেক্ষা করুন।",
	    )
	)
    };

    match create_reply(
	uid,
	tid,
	&reply_form.comment,
	state_pool,
    ).await {
	Ok(id) => {
	    Ok(Redirect::to(&format!("/submission/{}", id)))
	},
	Err(e) => {
	    eprintln!("Error submitting reply: {}", e);
	    Err(
		(
		    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	}
    }
}
