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
    ctx_up_sidebar,
    top_threads,
};

pub async fn root_page(
    State(pool): State<PgPool>,
) -> Result<Html<String>, axum::http::StatusCode> {
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

    ctx_up_sidebar(pool, &mut ctx).await;

    let rendered = TERA.render("root.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("Error rendering root page: {}", e);
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    Ok(Html(content))
}

pub async fn formatting_page (
    State(pool): State<PgPool>,
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();

    ctx_up_sidebar(pool, &mut ctx).await;

    let rendered = TERA.render("formatting_guide.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("{e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    Ok(Html(content))
}
