use axum::{
    extract::{ConnectInfo,State},
    Form,
    response::Html,
};
use sqlx::PgPool;
use std::error::Error;
use std::net::SocketAddr;

use crate::template::{
    Board_b_CTX as CTX,
    TERA,
};
use crate::forms::ThreadForm;
use crate::helper::{
    Thread,
    ThreadSerializable,
    create_thread,
};

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
    boardname: &str,
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();
    ctx.insert("board", &CTX.board);
    let threads = board_threads(
	boardname,
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

pub async fn board_b_page(
    State(pool): State<PgPool>,
) -> Result<Html<String>, axum::http::StatusCode> {
    board_page(
	pool,
	"/b/"
    ).await
}

pub async fn board_b_submission(
    State(pool): State<PgPool>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(thread_form): Form<ThreadForm>,
) -> &'static str {
    board_submission(
	pool,
	addr,
	thread_form,
	"/b/"
    ).await
}
