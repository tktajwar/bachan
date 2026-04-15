use axum::{
    routing::get,
    Router,
    response::Redirect,
};
use lazy_static::lazy_static;
use std::net::SocketAddr;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    services::ServeDir,
    timeout::TimeoutLayer,
};

mod pages;
use pages::*;
mod formatting;
mod moderation;

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
	.route("/", get(root_page).post(thread_submission))
	.route("/k/", get(k_page))
	.route("/k", get(|| async {Redirect::to("/k/")}))
	.route("/k/{tid_hex}", get(k_thread_page).post(reply_submission))
	.route("/mod/{tid_hex}", get(mod_id_page).post(mod_id_submission))
	.route("/{boardname}/", get(board_x_page).post(board_x_submission))
	.route("/{boardname}", get(board_x_page).post(board_x_submission))
	.route("/token", get(token_page).post(token_submission))
	.route(
	    "/register/{token_id}",
	    get(register_id_page).post(registeration_submission),
	)
	.route(
	    "/submission/{id}",
	    get(submission_id_page).post(confirmation_submission),
	)
	.route("/favicon.ico", get(|| async {
	    Redirect::permanent("static/favicon.ico")
	}))
	.fallback(fallback)
	.with_state(pool)
	.nest_service("/static", ServeDir::new("static"))
	.layer(
	    ServiceBuilder::new()
		.layer(TimeoutLayer::with_status_code(
		    axum::http::StatusCode::REQUEST_TIMEOUT,
		    Duration::from_secs(10),
		))
	)
	.layer(CompressionLayer::new());

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
	    .unwrap_or("123".to_string())
	    .parse()
	    .unwrap_or(123)
    };
}

const INTERNAL_SERVER_ERROR_REPLY: &str = "\
সার্ভারে একটি অভ্যন্তরীণ ত্রুটি ঘটেছে। অনুগ্রহ করে প্রশাসকের সাথে যোগাযোগ করুন।\
";

const USER_SUSPENDED_REPLY: &str = "\
আপনি বর্তমানে ফোরামে অংশগ্রহণ থেকে সাময়িকভাবে স্থগিত রয়েছেন।\
";
