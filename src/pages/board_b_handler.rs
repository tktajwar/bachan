use axum::{
    extract::{ConnectInfo,State},
    Form,
    response::Html,
};
use std::error::Error;
use std::net::SocketAddr;
use sqlx::PgPool;

use crate::template::{
    Board_b_CTX as CTX,
    TERA,
};
use crate::helper::{
    board_threads,
    create_thread,
};
use crate::forms::ThreadForm;

pub async fn board_b_page(
    State(pool): State<PgPool>,
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();
    ctx.insert("board", &CTX.board);
    let threads = board_threads(
	"/b/",
	pool,
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("threads", &threads);

    let rendered = TERA.render("board.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("{e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    Ok(Html(content))
}

pub async fn board_b_submission(
    State(pool): State<PgPool>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(thread_form): Form<ThreadForm>,
) -> &'static str {
    match create_thread(
	addr.ip(),
	thread_form.subject,
	thread_form.comment,
	"/b/".to_string(),
	pool,
    ).await {
	Ok(()) => "Hello! Your thread has been posted.",
	Err(e) => {
	    eprintln!("{}", e);
	    "Couldn't process query."
	}
    }
}
