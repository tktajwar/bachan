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

use crate::forms::TokenForm;
use crate::moderation::{
    create_mod_token,
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
