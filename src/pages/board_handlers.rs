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
use real::RealIp;
use sqlx::PgPool;
use std::error::Error;
use uuid::Uuid;

use crate::{
    INTERNAL_SERVER_ERROR_REPLY,
    USER_SUSPENDED_REPLY,
};
use crate::forms::{
    PaginationWithMTime,
    ThreadForm,
};
use crate::helper::{
    create_thread,
    ctx_up_sidebar,
    get_board_ctx,
    hashed,
    paginated_board_threads,
};
use crate::moderation::{
    is_g_open,
    is_user_suspended,
};
use crate::template::*;

async fn board_page (
    pool_state: State<PgPool>,
    pagination: axum::extract::Query<PaginationWithMTime>,
    url: &str,
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();

    let Ok(board) = get_board_ctx (
	url,
	pool_state.clone(),
    ).await else {
	return Err(axum::http::StatusCode::NOT_FOUND)
    };
    ctx.insert("board", &board);

    ctx_up_sidebar(pool_state.clone(), &mut ctx).await;

    if let Err(e) = paginated_board_threads (
	pagination.before_mtime,
	pagination.limit,
	url,
	pool_state.clone(),
    ).await {
	eprintln!("{}", e);
    };

    let (threads, has_more, limit, last_mtime) = paginated_board_threads (
	pagination.before_mtime,
	pagination.limit,
	url,
	pool_state,
    ).await.unwrap_or(
	(vec![], false, 0, 0)
    );
    ctx.insert("threads", &threads);

    ctx.insert("threads", &threads);
    ctx.insert("has_more", &has_more);
    ctx.insert("last_mtime", &last_mtime);
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

pub async fn board_submission(
    state_pool: State<PgPool>,
    uid: i64,
    thread_form: ThreadForm,
    boardname: &str,
) -> Result<Uuid, Box<dyn Error>> {
    let id = create_thread (
	uid,
	&thread_form.subject,
	&thread_form.comment,
	boardname.to_string(),
	state_pool,
    ).await?;

    Ok(id)
}

pub async fn board_x_page (
    state_pool: State<PgPool>,
    Path(url): Path<String>,
    pagination: axum::extract::Query<PaginationWithMTime>,
) -> impl IntoResponse {
    board_page(
	state_pool,
	pagination,
	&url,
    ).await
}

pub async fn board_x_submission (
    state_pool: State<PgPool>,
    Path(boardname): Path<String>,
    RealIp(ip): RealIp,
    Form(thread_form): Form<ThreadForm>,
) -> impl IntoResponse {
    let uid = hashed(ip);

    match is_user_suspended(uid, state_pool.clone()).await {
	Ok(true) => return Err(
	    (
		axum::http::StatusCode::FORBIDDEN,
		USER_SUSPENDED_REPLY,
	    ),
	),
	Ok(false) => (),
	Err(e) => {
	    eprintln!("Error checking user suspension: {}", e);
	    return Err(
		(
		    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	},
    }

    if boardname == "g" {
	let Ok(g_ok) = is_g_open().await else {
	    return Err(
		(
		    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	};

	if !g_ok {
	    return Err (
		(
		    axum::http::StatusCode::UNAUTHORIZED,
		    "আপনি এই বোর্ডে পোস্ট করতে পারবেন না।",
		)
	    )
	}
    }

    // let number_of_posts_by_user = match number_of_pending_posts_in_last_hour(
    // 	uid,
    // 	state_pool.clone(),
    // ).await {
    // 	Ok(number) => number,
    // 	Err(e) => {
    // 	    eprintln!("Error checking user pending posts number: {}", e);
    // 	    return Err(
    // 		(
    // 		    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
    // 		    INTERNAL_SERVER_ERROR_REPLY,
    // 		)
    // 	    )
    // 	},
    // };
    // if number_of_posts_by_user > 10 {
    // 	return Err(
    // 	    (
    // 		axum::http::StatusCode::TOO_MANY_REQUESTS,
    // 		"আপনার গত এক ঘণ্টায় অনেক অনির্বাচিত পোস্ট রয়েছে। পুনরায় পোস্ট \
    // 		 করার আগে অনুগ্রহ করে কিছুক্ষণ অপেক্ষা করুন।",
    // 	    )
    // 	)
    // };

    match board_submission(
	state_pool,
	uid,
	thread_form,
	&boardname,
    ).await {
	Ok(id) => {
	    Ok(Redirect::to(&format!("/submission/{}", id)))
	},
	Err(e) => {
	    eprintln!("Error submitting thread: {}", e);
	    Err(
		(
		    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	},
    }
}
