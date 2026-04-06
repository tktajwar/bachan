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
use crate::moderation::{
    create_mod_token,
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
