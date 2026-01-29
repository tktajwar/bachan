use axum::response::Html;
use tokio::fs;

pub async fn root_page() -> Result<Html<String>, axum::http::StatusCode> {
    let content = fs::read_to_string("templates/index.html")
	.await
	.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(content))
}
