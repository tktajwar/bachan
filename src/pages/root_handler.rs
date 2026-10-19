use axum::{
    extract::{
	State,
    },
    response::{
	Html,
    },
};
use sqlx::PgPool;
use std::collections::HashMap;

use crate::template::TERA;

use crate::model::{
    BoardStats,
    boards_stats,
    top_threads,
};

pub async fn root_page(
    State(pool): State<PgPool>,
    State(pages_cache): State<crate::PagesCache>,
) -> Result<Html<String>, axum::http::StatusCode> {
    if let Some(content) = pages_cache.get("root").await {
	return Ok(Html(content));
    }

    eprintln!("Building root page");

    let mut ctx = tera::Context::new();

    let threads = match top_threads (
	6,
	pool.clone(),
    ).await {
	Ok(result) => result,
	Err(e) => {
	    eprintln!("Error retrieving top threads: {}", e);
	    vec![]
	},
    };

    ctx.insert("threads", &threads);

    let boards = match boards_stats(pool).await {
	Ok(boards) => boards,
	Err(e) => {
	    eprintln!("Error processing boards stats: {}", e);
	    vec![]
	}
    };

    let activity_threshold = boards.iter()
	.map(|board| board.active_threads + board.new_replies)
	.max()
	.max(Some(50));

    let activity_threshold = activity_threshold.unwrap_or(50) / 10;
    eprintln!("activity threshold: {}", activity_threshold);

    let mut boards = boards
	.into_iter()
	.fold(HashMap::new(), |mut map: HashMap<String, Vec<BoardStats>>, board| {
	    map.entry(board.category.clone()).or_default().push(board);
	    map
	});

    let hobbies = boards.remove("Hobbies").unwrap_or_default();
    let interests = boards.remove("Interests").unwrap_or_default();
    let lifestyle = boards.remove("Lifestyle").unwrap_or_default();
    let local = boards.remove("Local").unwrap_or_default();
    let misc = boards.remove("Misc").unwrap_or_default();

    ctx.insert("hobbies", &hobbies);
    ctx.insert("interests", &interests);
    ctx.insert("lifestyle", &lifestyle);
    ctx.insert("local", &local);
    ctx.insert("misc", &misc);
    ctx.insert("activity_threshold", &activity_threshold);

    let rendered = TERA.render("root.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("Error rendering root page: {}", e);
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    pages_cache.insert("root".to_string(), content.clone()).await;

    Ok(Html(content))
}

pub async fn formatting_page (
    State(pages_cache): State<crate::PagesCache>,
) -> Result<Html<String>, axum::http::StatusCode> {
    if let Some(content) = pages_cache.get("formatting").await {
	return Ok(Html(content));
    }

    let ctx = tera::Context::new();

    let rendered = TERA.render("formatting_guide.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("{e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    pages_cache
	.insert("formatting".to_string(), content.clone())
	.await;

    Ok(Html(content))
}
