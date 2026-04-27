use axum::{
    extract::{
	State,
    },
    http::StatusCode,
    response::{
	Html,
	IntoResponse,
    },
};
use sqlx::PgPool;

use crate::{
    INTERNAL_SERVER_ERROR_REPLY,
};
use crate::helper::{
    ctx_up_sidebar,
    redacted_replies,
    redacted_threads,
};
use crate::template::*;

pub async fn redacted_threads_page (
    pool_state: State<PgPool>,
) -> impl IntoResponse {
    let mut ctx = tera::Context::new();

    ctx_up_sidebar(pool_state.clone(), &mut ctx).await;

    let threads = match redacted_threads (
	pool_state,
    ).await {
	Ok(redacted_threads) => redacted_threads,
	Err(e) => {
	    eprintln!("Error retrieving redacted threads: {}", e);
	    return Err((
		StatusCode::INTERNAL_SERVER_ERROR,
		INTERNAL_SERVER_ERROR_REPLY,
	    ))
	},
    };
    ctx.insert("threads", &threads);

    let rendered = TERA.render("redacted-threads.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("Failed to render redacted threads page: {}", e);
	    return Err((
		StatusCode::INTERNAL_SERVER_ERROR,
		INTERNAL_SERVER_ERROR_REPLY,
	    ))
	},
    };

    Ok(Html(content))
}

pub async fn redacted_replies_page (
    pool_state: State<PgPool>,
) -> impl IntoResponse {
    let mut ctx = tera::Context::new();

    ctx_up_sidebar(pool_state.clone(), &mut ctx).await;

    let replies = match redacted_replies (
	pool_state,
    ).await {
	Ok(redacted_replies) => redacted_replies,
	Err(e) => {
	    eprintln!("Error retrieving redacted replies: {}", e);
	    return Err((
		StatusCode::INTERNAL_SERVER_ERROR,
		INTERNAL_SERVER_ERROR_REPLY,
	    ))
	},
    };
    ctx.insert("replies", &replies);

    let rendered = TERA.render("redacted-replies.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("Failed to render redacted replies page: {}", e);
	    return Err((
		StatusCode::INTERNAL_SERVER_ERROR,
		INTERNAL_SERVER_ERROR_REPLY,
	    ))
	},
    };

    Ok(Html(content))
}
