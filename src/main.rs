use axum::{
    routing::get,
    Router,
};

mod pages;
use pages::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
	.route("/", get(root_page));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
