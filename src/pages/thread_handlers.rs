use axum::{
    Json,
    extract::{
	Path,
	State,
    },
    Form,
    response::{
	Html,
	IntoResponse,
	Redirect,
    },
    http::StatusCode,
};
use real::RealIp;
use sqlx::PgPool;

use crate::{
    INTERNAL_SERVER_ERROR_REPLY,
};
use crate::forms::{
    ThreadUpdates,
    ReplyForm,
};
use crate::template::{
    TERA,
};
use crate::model::{
    Post,
    PostSerialized,
    HexID,
    create_reply,
    ctx_up_sidebar,
    post_summary,
    thread_replies,
    thread_replies_after,
    post_with_id,
};

pub async fn thread_page (
    State(pool): State<PgPool>,
    State(boards_cache): State<crate::BoardsCache>,
    Path((board, HexID(id))): Path<(String, HexID)>,
) -> impl IntoResponse {
    let post = match post_with_id (id, pool.clone()).await {
	Ok(Some(post)) => post,
	Ok(None) => return Err(StatusCode::NOT_FOUND),
	Err(e) => {
	    eprintln!("Error finding post {}:\n{}", id, e);
	    return Err(StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    thread_page_view (
	pool,
	boards_cache,
	post,
	&board,
    ).await
}

async fn thread_page_view (
    pool: PgPool,
    boards_cache: crate::BoardsCache,
    post: Post,
    board: &str,
) -> Result<Result<Html<String>, Redirect>, StatusCode> {
    if let Some(thread_id) = post.thread_id {
	return Ok(Err(Redirect::permanent(
	    &format!("/{}/{:x}#{:x}", board, thread_id, post.id)
	)))
    } else if let Some(post_board) = &post.board {
	if post_board != board {
	    return Ok(Err(Redirect::permanent(
		&format!("/{}/{:x}", post_board, post.id)
	    )))
	}
    }

    let (replies, last_id) = match thread_replies (
	post.id,
	pool.clone(),
    ).await {
	Ok((replies, last_id)) => (replies, last_id),
	Err(e) => {
	    eprintln!("Error retrieving thread replies: {}", e);
	    return Err(StatusCode::INTERNAL_SERVER_ERROR)
	}
    };

    let replies: Vec<PostSerialized> = replies
	.into_iter()
	.map(|reply| PostSerialized::from(reply))
	.collect();

    let mut ctx = tera::Context::new();
    ctx_up_sidebar(pool, boards_cache, &mut ctx).await;
    ctx.insert("thread", &PostSerialized::from(post));
    ctx.insert("replies", &replies);
    ctx.insert("last_id", &last_id);
    let rendered = TERA.render("board_thread.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("{e}");
	    return Err(StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    Ok(Ok(Html(content)))
}

pub async fn reply_submission (
    State(pool): State<PgPool>,
    Path((board, HexID(thread_id))): Path<(String, HexID)>,
    RealIp(ip): RealIp,
    Form(reply_form): Form<ReplyForm>,
) -> impl IntoResponse {
    match create_reply(
	ip,
	thread_id,
	reply_form,
	pool,
    ).await {
	Ok(id) => {
	    Ok (
		Redirect::to (
		    &format!("/{}/{:x}#{:x}", board, thread_id, id)
		)
	    )
	},
	Err(e) => {
	    eprintln!("Error submitting reply: {}", e);
	    Err(
		(
		    StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	}
    }
}

pub async fn thread_updates (
    State(pool): State<PgPool>,
    Path((_board, HexID(tid))): Path<(String, HexID)>,
    thread_updates: axum::extract::Query<ThreadUpdates>,
) -> Result<Json<(Vec<PostSerialized>, i64)>, StatusCode> {
    let (replies, last_id) = thread_replies_after (
	tid,
	thread_updates.after_id,
	pool,
    ).await.unwrap_or(
	(vec![], thread_updates.after_id)
    );

    let replies: Vec<PostSerialized> = replies
	.into_iter()
	.map(|reply| PostSerialized::from(reply))
	.collect();

    Ok(Json((replies, last_id)))
}

pub async fn summary_page (
    State(pool): State<PgPool>,
    Path((_board, HexID(id))): Path<(String, HexID)>,
) -> Result<String, StatusCode> {
    let Ok(summary) = post_summary (
	id as i32,
	pool,
    ).await else {
	return Err(StatusCode::NOT_FOUND)
    };

    Ok(summary.comment)
}
