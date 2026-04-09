use axum::{
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

use crate::helper::{
    confirm_post,
    hashed,
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
		    "The submission was not found. \
		     Please recheck the URL.",
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
) -> impl IntoResponse {
    let uid = hashed(addr.ip());

    match is_user_suspended(uid, state_pool.clone()).await {
	Ok(true) => return Err(
	    (
		StatusCode::FORBIDDEN,
		"You are currently suspended.",
	    )
	),
	Ok(false) => (),
	Err(e) => {
	    eprintln!("Error checking user suspension: {}", e);
	    return Err(
		(
		    StatusCode::INTERNAL_SERVER_ERROR,
		    "There was an internal server error. \
		     Please contact the admin.",
		)
	    )
	},
    }

    let posted_id = match confirm_post(
	id,
	uid,
	state_pool,
    ).await {
	Ok(Some(id)) => id,
	Ok(None) => {
	    return Err (
		(
		    StatusCode::NOT_FOUND,
		    "The submission was not found. \
		     Please recheck the URL.",
		)
	    )
	}
	Err(e) => {
	    eprintln!("Error checking user suspension: {}", e);
	    return Err(
		(
		    StatusCode::INTERNAL_SERVER_ERROR,
		    "There was an internal server error. \
		     Please contact the admin.",
		)
	    )
	},
    };

    let tid_hex = format!("{:03x}", posted_id);

    Ok(Redirect::to(
	&format!("/k/{}", tid_hex)
    ))
}
