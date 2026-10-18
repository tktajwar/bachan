use axum::{
    extract::{
	State,
    },
    response::{
	Html,
    },
};
use sqlx::PgPool;

use crate::template::TERA;

use crate::model::{
    top_threads,
};

pub async fn root_page(
    State(pool): State<PgPool>,
    State(pages_cache): State<crate::PagesCache>,
) -> Result<Html<String>, axum::http::StatusCode> {
    if let Some(content) = pages_cache.get("root").await {
	return Ok(Html(content));
    }

    let mut ctx = tera::Context::new();

    let (threads, last_mtime) = match top_threads (
	24,
	pool.clone(),
    ).await {
	Ok(result) => result,
	Err(e) => {
	    eprintln!("Error retrieving top threads: {}", e);
	    (vec![], 0)
	},
    };
    ctx.insert("threads", &threads);
    ctx.insert("last_mtime", &last_mtime);

    let rendered = TERA.render("root.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("Error rendering root page: {}", e);
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    pages_cache.insert("root".to_string(), content.clone()).await;

    Ok(Html(content))
}

pub async fn formatting_page (
    State(pages_cache): State<crate::PagesCache>,
) -> Result<Html<String>, axum::http::StatusCode> {
    if let Some(content) = pages_cache.get("formatting").await {
	return Ok(Html(content));
    }

    let ctx = tera::Context::new();

    let rendered = TERA.render("formatting_guide.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("{e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    pages_cache
	.insert("formatting".to_string(), content.clone())
	.await;

    Ok(Html(content))
}
