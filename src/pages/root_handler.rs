use axum::{
    response::Html,
    extract::State,
};
use sqlx::PgPool;

use crate::template::TERA;
use crate::helper::{
    boards_in_category,
    top_announcements,
    top_threads,
};

pub async fn root_page(
    pool_state: State<PgPool>,
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();

    let hobbies = boards_in_category(
	"Hobbies",
	pool_state.clone(),
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("hobbies", &hobbies);

    let interests = boards_in_category(
	"Interests",
	pool_state.clone(),
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("interests", &interests);

    let lifestyle = boards_in_category(
	"Lifestyle",
	pool_state.clone(),
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("lifestyle", &lifestyle);

    let local = boards_in_category(
	"Local",
	pool_state.clone(),
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("local", &local);

    let misc = boards_in_category(
	"Misc",
	pool_state.clone(),
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("misc", &misc);

    let threads = top_threads (
	5,
	pool_state.clone(),
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("threads", &threads);

    let announcements = top_announcements (
	pool_state,
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("announcements", &announcements);

    let rendered = TERA.render("root.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(_) => {
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    Ok(Html(content))
}
