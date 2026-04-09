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
use sqlx::PgPool;

use crate::forms::ModerationForm;
use crate::moderation::{
    suspend_user,
    verify_mod,
};
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
    Form(moderation_form): Form<ModerationForm>,
) -> impl IntoResponse {
    match verify_mod(
	state_pool.clone(),
	&moderation_form.username,
	&moderation_form.passphrase,
    ).await {
	Ok(verification) => {
	    if verification == false {
		return Err (
		    (
			axum::http::StatusCode::UNAUTHORIZED,
			"Unauthorized! Recheck your username and passphrase.",
		    )
		)
	    }
	},
	Err(e) => {
	    eprintln!("Error validating moderator: {e}");
	    return Err (
		(
		    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
		    "\
		    The server had an internal error processing your\
		    credentials.\
		    ",
		)
	    )
	},
    };

    let Ok(id_u32) = u32::from_str_radix(&id_hex, 16) else {
	return Err (
	    (
		axum::http::StatusCode::BAD_REQUEST,
		"The server couldn't process the ID",
	    )
	)
    };
    let id = id_u32 as i32;

    if let Some(redact) = moderation_form.redact {
	if redact == "redact" {
	    if let Err(e) = redact_thread_or_reply (
		id,
		&moderation_form.username,
		&moderation_form.reason,
		state_pool.clone(),
	    ).await {
		eprintln!("Failed redacting post: {}", e);
		return Err (
		    (
			axum::http::StatusCode::INTERNAL_SERVER_ERROR,
			"Internal Server Error: Failed to redact",
		    )
		)
	    }
	}
    }

    if let Some(suspend) = moderation_form.suspend {
	if suspend == "suspend" {
	    if let Err(e) = suspend_user (
		id,
		&moderation_form.username,
		7,
		&moderation_form.reason,
		state_pool,
	    ).await {
		eprintln!("Failed suspending user: {}", e);
		return Err (
		    (
			axum::http::StatusCode::INTERNAL_SERVER_ERROR,
			"Internal Server Error: Failed to suspend",
		    )
		)
	    }
	}
    }

    Ok(Redirect::to(&format!("/k/{}", id_hex)))
}
