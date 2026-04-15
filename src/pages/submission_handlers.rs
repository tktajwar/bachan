use axum::{
    Form,
    extract::{
	ConnectInfo,
	Path,
	State,
    },
    response::{
	Html,
	IntoResponse,
	Redirect,
    },
    http::StatusCode,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use uuid::Uuid;

use crate::{
    INTERNAL_SERVER_ERROR_REPLY,
    USER_SUSPENDED_REPLY,
};
use crate::forms::SubmissionForm;
use crate::helper::{
    confirm_post,
    country_code,
    delete_pending_post,
    hashed,
    // number_of_replies_in_last_hour,
    number_of_threads_in_last_hour,
    pending_post_with_id,
};
use crate::moderation::is_user_suspended;
use crate::template::*;

pub async fn submission_id_page (
    state_pool: State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let pending_post = match pending_post_with_id (
	id,
	state_pool,
    ).await {
	Ok(Some(pending_post)) => pending_post,
	Ok(None) => {
	    return Err (
		(
		    StatusCode::NOT_FOUND,
		    "সাবমিশন আইডি পাওয়া যায়নি। অনুগ্রহ করে আপনার URL \
		     যাচাই করুন বা পুনরায় পোস্ট করুন।",
		)
	    )
	}
	Err(e) => {
	    eprintln!("Error retrieving pending post: {e}");
	    return Err (
		(
		    StatusCode::INTERNAL_SERVER_ERROR,
		    "There was an internal server error. \
		     Please contact the admin.",
		)
	    )
	}
    };

    let mut ctx = tera::Context::new();
    ctx.insert("pending_post", &pending_post);

    let rendered = TERA.render("submission_id.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("{e}");
	    return Err (
		(
		    StatusCode::INTERNAL_SERVER_ERROR,
		    "There was an internal server error. \
		     Please contact the admin.",
		)
	    )
	},
    };

    Ok(Html(content))
}

pub async fn confirmation_submission (
    state_pool: State<PgPool>,
    Path(id): Path<Uuid>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(submission): Form<SubmissionForm>,
) -> impl IntoResponse {
    let uid = hashed(addr.ip());

    match is_user_suspended(uid, state_pool.clone()).await {
	Ok(true) => return Err(
	    (
		StatusCode::FORBIDDEN,
		USER_SUSPENDED_REPLY,
	    )
	),
	Ok(false) => (),
	Err(e) => {
	    eprintln!("Error checking user suspension: {}", e);
	    return Err(
		(
		    StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	},
    }

    let country = match country_code(addr.ip()) {
	Ok(code) => code,
	Err(e) => {
	    eprintln!("Error checking country code: {}", e);
	    return Err(
		(
		    StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	},
    };

    match submission.action.as_str() {
	"submit" => Ok(confirm_submission(state_pool, id, uid, country).await),
	"cancel" => Ok(cancel_submission(state_pool, id).await),
	_ =>  Err(
	    (
		StatusCode::BAD_REQUEST,
		"ERROR 400: Bad Request! Invalid 'Action' value.",
	    )
	),
    }
}

async fn confirm_submission (
    state_pool: State<PgPool>,
    id: Uuid,
    uid: i64,
    country: String,
) -> Result<Redirect, (StatusCode, &'static str)> {
    let number_of_threads_by_user = match number_of_threads_in_last_hour(
	uid,
	state_pool.clone(),
    ).await {
	Ok(number) => number,
	Err(e) => {
	    eprintln!("Error checking user threads number: {}", e);
	    return Err(
		(
		    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	},
    };
    // let number_of_replies_by_user = match number_of_replies_in_last_hour(
    // 	uid,
    // 	state_pool.clone(),
    // ).await {
    // 	Ok(number) => number,
    // 	Err(e) => {
    // 	    eprintln!("Error checking user replies number: {}", e);
    // 	    return Err(
    // 		(
    // 		    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
    // 		    INTERNAL_SERVER_ERROR_REPLY,
    // 		)
    // 	    )
    // 	},
    // };
    if number_of_threads_by_user >= 3 {
	return Err(
	    (
		axum::http::StatusCode::TOO_MANY_REQUESTS,
		"আপনি গত এক ঘণ্টায় অতিমাত্রায় নিবন্ধ পোস্ট করেছেন। পুনরায় পোস্ট \
		 করার আগে অনুগ্রহ করে কিছুক্ষণ অপেক্ষা করুন।",
	    )
	)
    };
    // if number_of_replies_by_user >= 15 {
    // 	return Err(
    // 	    (
    // 		axum::http::StatusCode::TOO_MANY_REQUESTS,
    // 		"আপনি গত এক ঘণ্টায় অতিমাত্রায় মন্তব্য পোস্ট করেছেন। পুনরায় পোস্ট \
    // 		 করার আগে অনুগ্রহ করে কিছুক্ষণ অপেক্ষা করুন।",
    // 	    )
    // 	)
    // };

    let posted_id = match confirm_post(
	id,
	uid,
	country,
	state_pool,
    ).await {
	Ok(Some(id)) => id,
	Ok(None) => {
	    return Err (
		(
		    StatusCode::NOT_FOUND,
		    "সাবমিশন আইডি পাওয়া যায়নি। অনুগ্রহ করে আপনার URL \
		     যাচাই করুন বা পুনরায় পোস্ট করুন।",
		)
	    )
	}
	Err(e) => {
	    eprintln!("Error checking user suspension: {}", e);
	    return Err(
		(
		    StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	},
    };

    let tid_hex = format!("{:03x}", posted_id);

    Ok(Redirect::to(
	&format!("/k/{}", tid_hex)
    ))
}

async fn cancel_submission (
    state_pool: State<PgPool>,
    id: Uuid,
) -> Result<Redirect, (StatusCode, &'static str)> {
    let Ok(_) = delete_pending_post(id, state_pool).await else {
	return Err(
	    (
		StatusCode::INTERNAL_SERVER_ERROR,
		INTERNAL_SERVER_ERROR_REPLY,
	    )
	)
    };

    Ok(Redirect::to(
	"/"
    ))
}
