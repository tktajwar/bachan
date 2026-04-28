use axum::{
    Json,
    Form,
    extract::{
	ConnectInfo,
	State,
    },
    response::{
	Html,
	IntoResponse,
	Redirect,
    },
};
use std::net::SocketAddr;
use sqlx::PgPool;

use crate::{
    INTERNAL_SERVER_ERROR_REPLY,
    USER_SUSPENDED_REPLY,
};
use crate::board_handlers::board_submission;
use crate::moderation::is_user_suspended;
use crate::forms::{
    HighlightsUpdates,
    PopupThreadForm,
    ThreadForm,
};
use crate::template::TERA;
use crate::helper::{
    TopThread,
    ctx_up_sidebar,
    hashed,
    highlights_updates,
    top_threads,
};

pub async fn root_page(
    pool_state: State<PgPool>,
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();

    let (threads, last_mtime) = match top_threads (
	24,
	pool_state.clone(),
    ).await {
	Ok(result) => result,
	Err(e) => {
	    eprintln!("Error retrieving top threads: {}", e);
	    (vec![], 0)
	},
    };
    ctx.insert("threads", &threads);
    ctx.insert("last_mtime", &last_mtime);

    ctx_up_sidebar(pool_state, &mut ctx).await;

    let rendered = TERA.render("root.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("Error rendering root page: {}", e);
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    Ok(Html(content))
}

pub async fn thread_submission (
    state_pool: State<PgPool>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(popup_thread_form): Form<PopupThreadForm>,
) -> impl IntoResponse {
    let uid = hashed(addr.ip());

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

    match board_submission (
	state_pool,
	uid,
	ThreadForm {
	    subject: popup_thread_form.subject,
	    comment: popup_thread_form.comment,
	},
	&popup_thread_form.board,
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

pub async fn root_updates (
    state_pool: State<PgPool>,
    updates: axum::extract::Query<HighlightsUpdates>,
) -> Json<(Vec<TopThread>, i64)> {
    let (threads, last_mtime) = highlights_updates (
	updates.after_mtime,
	state_pool,
    ).await.unwrap_or(
	(vec![], 0)
    );

    Json((threads, last_mtime))
}
