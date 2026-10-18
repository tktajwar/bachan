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

    eprintln!("Building root page");

    let mut ctx = tera::Context::new();

    let threads = match top_threads (
	6,
	pool.clone(),
    ).await {
	Ok(result) => result,
	Err(e) => {
	    eprintln!("Error retrieving top threads: {}", e);
	    vec![]
	},
    };
    ctx.insert("threads", &threads);

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
