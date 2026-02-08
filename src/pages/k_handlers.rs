use axum::{
    extract::State,
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
