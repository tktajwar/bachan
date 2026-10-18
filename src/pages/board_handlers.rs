use axum::{
    Form,
    extract::{
	Path,
	State,
    },
    response::{
	Html,
	IntoResponse,
	Redirect,
    },
};
use axum::http::StatusCode;
use real::RealIp;
use sqlx::PgPool;

use crate::{
    SUSPENDED_REPLY,
    INTERNAL_SERVER_ERROR_REPLY,
};

use crate::forms::{
    PaginationWithMTime,
    ThreadForm,
};

use crate::model::{
    create_thread,
    get_board_ctx,
    paginated_board_threads,
};

use crate::moderation::is_user_suspended;

use crate::template::*;

async fn board_page (
    pool: PgPool,
    pagination: axum::extract::Query<PaginationWithMTime>,
    url: &str,
) -> Result<Html<String>, StatusCode> {
    let mut ctx = tera::Context::new();

    let Ok(board) = get_board_ctx (
	url,
	pool.clone(),
    ).await else {
	return Err(axum::http::StatusCode::NOT_FOUND)
    };
    ctx.insert("board", &board);

    let (threads, has_more, limit, last_bump_time) = paginated_board_threads (
	pagination.before_mtime,
	pagination.limit,
	url,
	pool,
    ).await.unwrap_or(
	(vec![], false, 0, 0)
    );

    ctx.insert("threads", &threads);
    ctx.insert("has_more", &has_more);
    ctx.insert("last_bump_time", &last_bump_time);
    ctx.insert("limit", &limit);


    let rendered = TERA.render("board.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("Failed to render board page: {}", e);
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    Ok(Html(content))
}

pub async fn board_x_page (
    State(pool): State<PgPool>,
    Path(url): Path<String>,
    pagination: axum::extract::Query<PaginationWithMTime>,
) -> impl IntoResponse {
    board_page(
	pool,
	pagination,
	&url,
    ).await
}

pub async fn board_x_submission (
    State(pool): State<PgPool>,
    State(pages_cache): State<crate::PagesCache>,
    State(suspended_cache): State<crate::SuspendedCache>,
    Path(board): Path<String>,
    RealIp(ip): RealIp,
    Form(thread_form): Form<ThreadForm>,
) -> impl IntoResponse {
    match is_user_suspended (
	ip,
	pool.clone(),
	suspended_cache,
    ).await {
	Ok(true) => return Err (
	    (
		StatusCode::FORBIDDEN,
		SUSPENDED_REPLY,
	    )
	),
	Ok(false) => (),
	Err(e) => {
	    eprintln!("Error checking user suspension: {}", e);
	    return Err (
		(
		    StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	},
    };

    pages_cache.invalidate("root").await;

    match create_thread (
	ip,
	&thread_form.subject,
	&thread_form.comment,
	thread_form.name,
	&board,
	pool,
    ).await {
	Ok(id) => {
	    Ok(Redirect::to(&format!("/{}/{:3x}", board, id)))
	},
	Err(e) => {
	    eprintln!("Error submitting thread: {}", e);
	    Err (
		(
		    StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	},
    }
}
