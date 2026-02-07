use axum::{
    routing::get,
    Router,
    response::Redirect,
};
use lazy_static::lazy_static;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod pages;
use pages::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
	.expect("DATABASE_URL not set.");
    let pool = sqlx::postgres::PgPool::connect(&db_url).await
	.expect("Couldn't create database pool.");

    sqlx::migrate!("./migrations").run(&pool).await
	.expect("Couldn't do database migration.");

    let app = Router::new()
	.route("/", get(root_page))
	.route("/b/", get(board_b_page).post(board_b_submission))
	.route("/b", get(|| async {Redirect::to("/b/")}))
	.fallback(fallback)
	.with_state(pool)
	.nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(
	listener,
	app.into_make_service_with_connect_info::<SocketAddr>(),
    ).await.unwrap();
}

lazy_static! {
    pub static ref SECRET_NUMBER: u32 = {
	eprintln!("Reading secret number.");
	std::env::var("SECRET_NUMBER")
	    .unwrap_or(return 123)
	    .parse()
	    .unwrap_or(123)
    };
}
