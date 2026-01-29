use axum::response::Html;
use axum::http::{
    StatusCode,
    Uri,
};

pub async fn fallback(uri: Uri) -> (StatusCode, Html<String>) {
    (StatusCode::NOT_FOUND, Html(format!(
	"No route for {uri}. Go back to <a href='/'>homepage</a>?"
    )))
}
