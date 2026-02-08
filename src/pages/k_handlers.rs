use axum::{
    extract::{
	Path,
	State,
    },
    response::Html,
};
use std::error::Error;
use sqlx::PgPool;

use crate::template::{
    TERA,
};
use crate::helper::{
    Thread,
    ThreadSerializable,
};

async fn all_threads(
    pool: PgPool,
) -> Result<Vec<ThreadSerializable>, Box<dyn Error>> {
    let q = "\
    SELECT id, uid, subject, comment, board, ctime, mtime FROM thread \
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
    SELECT id, uid, subject, comment, board, ctime, mtime FROM thread \
    WHERE id = $1 \
    ORDER BY mtime desc \
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
	pool,
    ).await else {
	return Err(axum::http::StatusCode::NOT_FOUND)
    };

    let mut ctx = tera::Context::new();
    ctx.insert("thread", &thread);

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
