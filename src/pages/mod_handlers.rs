use axum::{
    extract::{
	Path,
	State,
    },
    response::{
	Html,
    },
};
use sqlx::PgPool;

use crate::template::{
    TERA,
};
use crate::helper::thread_or_reply_with_id;

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
