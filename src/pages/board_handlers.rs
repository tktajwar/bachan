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
use std::error::Error;
use std::net::SocketAddr;

use crate::forms::ThreadForm;
use crate::helper::{
    Thread,
    ThreadSerializable,
    create_thread,
    get_board_ctx,
    hashed,
};
use crate::moderation::is_user_suspended;
use crate::template::*;

pub async fn board_threads(
    board: &str,
    State(pool): State<PgPool>,
) -> Result<Vec<ThreadSerializable>, Box<dyn Error>> {
    let q = "\
    SELECT \
    t.id, \
    t.uid, \
    t.subject, \
    t.comment, \
    t.board, \
    t.ctime, \
    t.mtime, \
    t.redacted, \
    COUNT(r.id) AS reply_count \
    FROM thread t \
    LEFT JOIN reply r ON r.tid = t.id \
    WHERE t.board = $1 \
    GROUP BY t.id \
    ORDER BY mtime desc \
    ";

    let threads = sqlx::query_as::<_, Thread>(q)
	.bind(board)
	.fetch_all(&pool)
	.await?;

    let serializable_threads: Vec<ThreadSerializable> = threads.into_iter()
        .map(Thread::into_serializable)
        .collect();

    Ok(serializable_threads)
}

async fn board_page(
    state_pool: State<PgPool>,
    url: &str,
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();
    let Ok(board) = get_board_ctx(
	url,
	state_pool.clone(),
    ).await else {
	return Err(axum::http::StatusCode::NOT_FOUND)
    };

    ctx.insert("board", &board);
    let threads = board_threads(
	url,
	state_pool,
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("threads", &threads);

    let rendered = TERA.render("board.html", &ctx);
    let content = match rendered {
	Ok(s) => s,
	Err(e) => {
	    eprintln!("{e}");
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    };

    Ok(Html(content))
}

async fn board_submission(
    state_pool: State<PgPool>,
    uid: i32,
    thread_form: ThreadForm,
    boardname: &str,
) -> Result<i32, Box<dyn Error>> {
    let id = create_thread(
	uid,
	&thread_form.subject,
	&thread_form.comment,
	boardname.to_string(),
	state_pool,
    ).await?;

    Ok(id)
}

pub async fn board_x_page(
    state_pool: State<PgPool>,
    Path(url): Path<String>,
) -> Result<Html<String>, axum::http::StatusCode> {
    board_page(
	state_pool,
	&url,
    ).await
}

pub async fn board_x_submission(
    state_pool: State<PgPool>,
    Path(boardname): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(thread_form): Form<ThreadForm>,
) -> Result<Redirect, axum::http::StatusCode> {
    let uid = hashed(addr.ip());

    match is_user_suspended(uid, state_pool.clone()).await {
	Ok(true) => return Err(axum::http::StatusCode::FORBIDDEN),
	Ok(false) => (),
	Err(e) => {
	    eprintln!("Error checking user suspension: {}", e);
	    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    }

    match board_submission(
	state_pool,
	uid,
	thread_form,
	&boardname,
    ).await {
	Ok(id) => {
	    let id_hex = format!("{:03x}", id);
	    Ok(Redirect::to(&format!("/k/{}", id_hex)))
	},
	Err(e) => {
	    eprintln!("Error submitting thread: {}", e);
	    Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
	},
    }
}
