use axum::response::Html;
use tokio::fs;

use crate::dashboard_data::{
    CTX,
    TERA,
};

pub async fn root_page() -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();
    ctx.insert("boards", &CTX.boards);
    let rendered = TERA.render("index.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("{e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)}
	,
    };

    Ok(Html(content))
}
