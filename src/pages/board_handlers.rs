use axum::{
    Form,
    extract::{
	ConnectInfo,
	Path,
	State,
    },
    response::Html,
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
};
use crate::template::*;

pub async fn board_threads(
    board: &str,
    pool: PgPool,
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
    pool: PgPool,
    url: &str,
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();
    let Ok(board) = get_board_ctx(
	url,
	pool.clone(),
    ).await else {
	return Err(axum::http::StatusCode::NOT_FOUND)
    };

    ctx.insert("board", &board);
    let threads = board_threads(
	url,
	pool,
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
    pool: PgPool,
    addr: SocketAddr,
    thread_form: ThreadForm,
    boardname: &str,
) -> &'static str {
    match create_thread(
	addr.ip(),
	thread_form.subject,
	thread_form.comment,
	boardname.to_string(),
	pool,
    ).await {
	Ok(()) => "Hello! Your thread has been posted.",
	Err(e) => {
	    eprintln!("{}", e);
	    "Couldn't process query."
	}
    }
}

pub async fn board_x_page(
    State(pool): State<PgPool>,
    Path(url): Path<String>,
) -> Result<Html<String>, axum::http::StatusCode> {
    board_page(
	pool,
	&url,
    ).await
}

pub async fn board_x_submission(
    State(pool): State<PgPool>,
    Path(boardname): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(thread_form): Form<ThreadForm>,
) -> &'static str {
    board_submission(
	pool,
	addr,
	thread_form,
	&boardname,
    ).await
}
