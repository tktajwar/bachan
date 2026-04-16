use axum::{
    extract::{
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
use real::RealIp;
use sqlx::PgPool;

use crate::{
    INTERNAL_SERVER_ERROR_REPLY,
    USER_SUSPENDED_REPLY,
};
use crate::forms::{
    PaginationWithID,
    ReplyForm,
};
use crate::template::{
    TERA,
};
use crate::helper::{
    create_reply,
    hashed,
    number_of_pending_posts_in_last_hour,
    paginated_threads,
    thread_with_id,
    thread_with_reply_id,
};
use crate::moderation::is_user_suspended;

pub async fn k_page (
    state_pool: State<PgPool>,
    pagination: axum::extract::Query<PaginationWithID>,
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();

    let before = if let Some(id_hex) = &pagination.before_id {
	let Ok(id) = i32::from_str_radix(&id_hex, 16) else {
	    return Err(axum::http::StatusCode::BAD_REQUEST)
	};
	Some(id)
    } else {
	None
    };

    let (threads, has_more, limit) = paginated_threads(
	before,
	pagination.limit,
	state_pool,
    ).await.unwrap_or(
	(vec![], false, 0)
    );
    ctx.insert("threads", &threads);
    ctx.insert("has_more", &has_more);
    if let Some(last_thread) = threads.last() {
	ctx.insert("last_thread_id", &last_thread.id);
    }
    ctx.insert("limit", &limit);

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
    RealIp(ip): RealIp,
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

    let uid = hashed(ip);

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
