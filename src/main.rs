use axum::{
    Router,
    extract::State,
    middleware::Next,
    http::StatusCode,
    response::{Response,Redirect},
    routing::get,
};
use axum::extract::{Request,FromRef};
use lazy_static::lazy_static;
use moka::future::Cache;
use real::{RealIpLayer,IpAddr,RealIp};
use sqlx::postgres::PgPool;
use std::collections::HashSet;
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
mod moderation;

type PagesCache = moka::future::Cache<String, String>;
type SuspendedCache = moka::future::Cache<(), HashSet<IpAddr>>;

#[derive(Clone, FromRef)]
struct AppState {
    pool: PgPool,
    page_cache: PagesCache,
    suspended_cache: SuspendedCache,
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

const SUSPENDED_REPLY: &str = "\
আপনি এই সার্ভার ব্যবহার করা থেকে নিলম্বিত হয়েছেন।\
";

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
	.expect("DATABASE_URL not set.");
    let pool = PgPool::connect(&db_url).await
	.expect("Couldn't create database pool.");

    sqlx::migrate!("./migrations").run(&pool).await
	.expect("Couldn't do database migration.");

    let page_cache: PagesCache = Cache::builder()
	.time_to_live(Duration::from_hours(18))
	.max_capacity(500)
	.build();

    let suspended_cache: SuspendedCache = Cache::builder()
	.time_to_live(Duration::from_hours(18))
	.max_capacity(1)
	.build();

    let state = AppState {
	pool,
	page_cache,
	suspended_cache,
    };

    let app = Router::<AppState>::new()
	.route("/", get(root_page))
	.route("/{board}/{tid_hex}",
	       get(thread_page).post(reply_submission))
	.route("/{board}/moderation/{tid_hex}",
	       get(mod_id_page).post(mod_id_submission))
	.route("/{board}/{tid_hex}/updates", get(thread_updates))
	.route("/{board}/", get(board_x_page).post(board_x_submission))
	.route("/token", get(token_page).post(token_submission))
	.route(
	    "/register/{token_id}",
	    get(register_id_page).post(registeration_submission),
	)
	.route("/formatting", get(formatting_page))
	.route("/formatting/", get(formatting_page))
	.route("/favicon.ico", get(|| async {
	    Redirect::permanent("static/favicon.ico")
	}))
	.fallback(fallback)
	.layer(
	    axum::middleware::from_fn_with_state(
		state.clone(),
		forbid_suspeneded_ips,
	    )
	)
	.with_state(state)
	.nest_service("/static", ServeDir::new("static"))
	.layer(
	    ServiceBuilder::new()
		.layer(TimeoutLayer::with_status_code(
		    axum::http::StatusCode::REQUEST_TIMEOUT,
		    Duration::from_secs(10),
		))
	)
	.layer(RealIpLayer::default())
	.layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(
	listener,
	app.into_make_service_with_connect_info::<SocketAddr>(),
    ).await.unwrap();
}

async fn forbid_suspeneded_ips (
    RealIp(ip): RealIp,
    State(pool): State<PgPool>,
    State(suspended_cache): State<SuspendedCache>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Ok(true) = moderation::is_user_suspended (
	ip,
	pool,
	suspended_cache,
    ).await {
	return Err(
	    StatusCode::FORBIDDEN
	)
    }

    Ok(next.run(request).await)
}
