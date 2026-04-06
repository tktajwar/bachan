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
use uuid::Uuid;

use crate::forms::{
    TokenForm,
    RegisterationForm,
};
use crate::moderation::{
    create_mod_token,
    delete_mod_token,
    register_mod,
    verify_admin,
    verify_mod_token,
};
use crate::template::{
    TERA,
};

pub async fn token_page () -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();

    let rendered = TERA.render("mod_token.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("{e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    Ok(Html(content))
}

pub async fn token_submission (
    state_pool: State<PgPool>,
    Form(token_form): Form<TokenForm>,
) -> Result<String, axum::http::StatusCode> {
    match verify_admin(
	&token_form.server_pin,
	&token_form.server_passphrase,
    ).await {
	Ok(verification) => {
	    if verification == false {
		return Err(axum::http::StatusCode::UNAUTHORIZED)
	    }
	},
	Err(e) => {
	    eprintln!("Error validating admin: {e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    let Ok(token_id) = create_mod_token(
	state_pool,
	&token_form.token_passphrase.as_bytes(),
    ).await else {
	return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    };

    Ok( token_id.to_string() )
}

pub async fn register_id_page (
    Path(token_id): Path<Uuid>
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();
    ctx.insert("id", &token_id.to_string());

    let rendered = TERA.render("register_token.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("{e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    Ok(Html(content))
}

pub async fn registeration_submission (
    state_pool: State<PgPool>,
    Path(token_id): Path<Uuid>,
    Form(registeration_form): Form<RegisterationForm>,
) -> Result<String, axum::http::StatusCode> {
    match verify_mod_token (
	state_pool.clone(),
	token_id,
	&registeration_form.token_passphrase,
    ).await {
	Ok(verification) => {
	    if verification == false {
		return Err(axum::http::StatusCode::UNAUTHORIZED)
	    }
	},
	Err(e) => {
	    eprintln!("Error validating token: {e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    match delete_mod_token (
	state_pool.clone(),
	token_id,
    ).await {
	Ok(_) => {},
	Err(e) => {
	    eprintln!("Error deleting token: {e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    let id = match register_mod (
	state_pool,
	&registeration_form.username,
	&registeration_form.passphrase.as_bytes(),
    ).await {
	Ok(id) => id,
	Err(e) => {
	    eprintln!("Error registering mod: {e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    Ok( format!("Moderator {id} has been registered.") )
}
