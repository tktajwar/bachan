use axum::{
    extract::{
	ConnectInfo,
	Path,
	State,
    },
    Form,
    response::Html,
};
use std::error::Error;
use std::net::SocketAddr;
use sqlx::PgPool;

use crate::forms::ReplyForm;
use crate::template::{
    TERA,
};
use crate::helper::{
    Thread,
    ThreadSerializable,
    create_reply,
};

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
    COUNT(r.id) AS reply_count \
    FROM thread t \
    LEFT JOIN reply r ON r.tid = t.id \
    GROUP BY t.id \
    ORDER BY mtime desc \
    ";

    let threads = sqlx::query_as::<_, Thread>(q)
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
    COUNT(r.id) AS reply_count \
    FROM thread t \
    LEFT JOIN reply r ON r.tid = t.id \
    WHERE t.id = $1 \
    GROUP BY t.id \
    ";

    let thread = sqlx::query_as::<_, Thread>(q)
	.bind(&id)
	.fetch_one(&pool)
	.await?;

    let serializable_thread = thread.into_serializable();

    Ok(serializable_thread)
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
    Path(tid_hex): Path<String>,
) -> Result<Html<String>, axum::http::StatusCode> {
    let Ok(tid_u32) = u32::from_str_radix(&tid_hex, 16) else {
	return Err(axum::http::StatusCode::BAD_REQUEST)
    };
    let tid = tid_u32 as i32;

    let Ok(thread) = thread_with_id(
	tid,
	pool.clone(),
    ).await else {
	return Err(axum::http::StatusCode::NOT_FOUND)
    };

    let Ok(replies) = crate::helper::thread_replies(
	tid,
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

    Ok(Html(content))
}

pub async fn reply_submission(
    State(pool): State<PgPool>,
    Path(tid_hex): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(reply_form): Form<ReplyForm>,
) -> Result<&'static str, axum::http::StatusCode> {
    let Ok(tid_u32) = u32::from_str_radix(&tid_hex, 16) else {
	return Err(axum::http::StatusCode::BAD_REQUEST)
    };
    let tid = tid_u32 as i32;

    match create_reply(
	addr.ip(),
	tid,
	reply_form.comment,
	pool,
    ).await {
	Ok(()) => Ok("Hello! Your reply has been posted."),
	Err(e) => {
	    eprintln!("{}", e);
	    Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	}
    }
}
