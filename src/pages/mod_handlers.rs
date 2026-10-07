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
use crate::model::{
    HexID,
    ctx_up_sidebar,
    redact_thread_or_reply,
    thread_or_reply_with_id,
};

pub async fn mod_id_page(
    State(pool): State<PgPool>,
    State(boards_cache): State<crate::BoardsCache>,
    Path((_board, HexID(id))): Path<(String, HexID)>,
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();
    ctx_up_sidebar(pool.clone(), boards_cache, &mut ctx).await;

    let Ok(thread_or_reply) = thread_or_reply_with_id(
	id as i32,
	pool,
    ).await else {
	return Err(axum::http::StatusCode::NOT_FOUND)
    };
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
    State(pool): State<PgPool>,
    Path(id_hex): Path<String>,
    Form(moderation_form): Form<ModerationForm>,
) -> impl IntoResponse {
    match verify_mod(
	pool.clone(),
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
		pool.clone(),
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
		pool,
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
