use axum::{
    Form,
    response::Html,
};

use crate::template::{
    Board_b_CTX as CTX,
    TERA,
};

use crate::forms::ThreadForm;

pub async fn board_b_page() -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();
    ctx.insert("board", &CTX.board);
    let rendered = TERA.render("board.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("{e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)}
	,
    };

    Ok(Html(content))
}

pub async fn board_b_submission(
    Form(thread_form): Form<ThreadForm>,
) -> &'static str {
    println!("{}", thread_form.subject);
    println!("{}", thread_form.comment);
    "Hello"
}
