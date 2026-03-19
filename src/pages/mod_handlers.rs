use axum::{
    Form,
    extract::{
	ConnectInfo,
	Path,
	State,
    },
    response::{
	Html,
	Redirect,
    },
};
use sqlx::PgPool;
use std::net::SocketAddr;

use crate::forms::ModerationForm;
use crate::moderation::verify_mod;
use crate::template::{
    TERA,
};
use crate::helper::{
    redact_thread_or_reply,
    thread_or_reply_with_id,
};

pub async fn mod_id_page(
    state_pool: State<PgPool>,
    Path(id_hex): Path<String>,
) -> Result<Html<String>, axum::http::StatusCode> {
    let Ok(id_u32) = u32::from_str_radix(&id_hex, 16) else {
	return Err(axum::http::StatusCode::BAD_REQUEST)
    };
    let id = id_u32 as i32;

    let Ok(thread_or_reply) = thread_or_reply_with_id(
	id,
	state_pool,
    ).await else {
	return Err(axum::http::StatusCode::NOT_FOUND)
    };

    let mut ctx = tera::Context::new();
    ctx.insert("thread_or_reply", &thread_or_reply);

    let rendered = TERA.render("mod_id.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("{e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    Ok(Html(content))
}

pub async fn mod_id_submission(
    state_pool: State<PgPool>,
    Path(id_hex): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(moderation_form): Form<ModerationForm>,
) -> Result<Redirect, axum::http::StatusCode> {
    match verify_mod(
	state_pool.clone(),
	&moderation_form.username,
	&moderation_form.passphrase,
    ).await {
	Ok(verification) => {
	    if verification == false {
		return Err(axum::http::StatusCode::UNAUTHORIZED)
	    }
	},
	Err(e) => {
	    eprintln!("Error validating moderator: {e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    let Ok(id_u32) = u32::from_str_radix(&id_hex, 16) else {
	return Err(axum::http::StatusCode::BAD_REQUEST)
    };
    let id = id_u32 as i32;

    if let Some(redact) = moderation_form.redact {
	if redact == "redact" {
	    if let Err(e) = redact_thread_or_reply (
		id,
		&moderation_form.username,
		&moderation_form.reason,
		state_pool,
	    ).await {
		eprintln!("Failed redacting post: {}", e);
		return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	    }
	}
    }

    Ok(Redirect::to(&format!("/k/{}", id_hex)))
}
