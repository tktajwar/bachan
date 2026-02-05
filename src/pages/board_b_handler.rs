use axum::response::Html;

use crate::template::{
    Board_b_CTX as CTX,
    TERA,
};

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
