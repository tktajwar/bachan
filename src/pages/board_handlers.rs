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
    INTERNAL_SERVER_ERROR_REPLY,
};

use crate::forms::{
    PaginationWithMTime,
    ThreadForm,
};

use crate::model::{
    create_thread,
    ctx_up_sidebar,
    get_board_ctx,
    paginated_board_threads,
};

use crate::template::*;

async fn board_page (
    pool: PgPool,
    boards_cache: crate::BoardsCache,
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

    ctx_up_sidebar(pool.clone(), boards_cache, &mut ctx).await;

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
    State(boards_cache): State<crate::BoardsCache>,
    Path(url): Path<String>,
    pagination: axum::extract::Query<PaginationWithMTime>,
) -> impl IntoResponse {
    board_page(
	pool,
	boards_cache,
	pagination,
	&url,
    ).await
}

pub async fn board_x_submission (
    State(pool): State<PgPool>,
    Path(board): Path<String>,
    RealIp(ip): RealIp,
    Form(thread_form): Form<ThreadForm>,
) -> impl IntoResponse {
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
