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
    post_with_id,
    restore_post,
    redact_post,
    thread_id_of,
};

pub async fn mod_id_page(
    State(pool): State<PgPool>,
    State(boards_cache): State<crate::BoardsCache>,
    Path((_board, HexID(id))): Path<(String, HexID)>,
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();
    ctx_up_sidebar(pool.clone(), boards_cache, &mut ctx).await;

    let post = match post_with_id(
	id,
	pool,
    ).await {
	Ok(Some(post)) => post,
	Ok(None) => {
	    return Err(axum::http::StatusCode::NOT_FOUND)
	},
	Err(e) => {
	    eprintln!("Error retrieving post for moderation: {}", e);
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    let post = crate::model::PostSerialized::from(post);
    ctx.insert("post", &post);

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
    State(suspended_cache): State<crate::SuspendedCache>,
    State(pages_cache): State<crate::PagesCache>,
    Path((board, HexID(id))): Path<(String, HexID)>,
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

    let thread_id = match thread_id_of(id, pool.clone()).await {
	Ok(Some(thread_id)) => thread_id,
	_ => id,
    };

    pages_cache.invalidate(&format!("/{board}/{thread_id:03x}")).await;

    if let Some(restore) = moderation_form.restore {
	if restore == "restore" {
	    if let Err(e) = restore_post (
		id,
		pool.clone(),
		suspended_cache,
	    ).await {
		eprintln!("Failed restoring post: {}", e);
		return Err (
		    (
			axum::http::StatusCode::INTERNAL_SERVER_ERROR,
			"Internal Server Error: Failed to redact",
		    )
		)
	    }
	}

	return Ok(Redirect::to(&format!("/{}/{:3x}#{:3x}", board, thread_id, id)))
    };

    if let Some(redact) = moderation_form.redact {
	if redact == "redact" {
	    if let Err(e) = redact_post (
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
		suspended_cache,
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

    Ok(Redirect::to(&format!("/{}/{:3x}#{:3x}", board, thread_id, id)))
}
