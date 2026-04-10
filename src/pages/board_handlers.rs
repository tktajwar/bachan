use axum::{
    Form,
    extract::{
	ConnectInfo,
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
use std::error::Error;
use std::net::SocketAddr;
use uuid::Uuid;

use crate::{
    INTERNAL_SERVER_ERROR_REPLY,
    USER_SUSPENDED_REPLY,
};
use crate::forms::ThreadForm;
use crate::helper::{
    Thread,
    ThreadSerializable,
    create_thread,
    get_board_ctx,
    hashed,
    list_of_boards,
    number_of_pending_posts_in_last_hour,
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
    AND t.redacted = false \
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

async fn board_page (
    state_pool: State<PgPool>,
    url: &str,
) -> Result<Html<String>, axum::http::StatusCode> {
    let mut ctx = tera::Context::new();

    let Ok(board) = get_board_ctx (
	url,
	state_pool.clone(),
    ).await else {
	return Err(axum::http::StatusCode::NOT_FOUND)
    };
    ctx.insert("board", &board);

    let boards = list_of_boards (
	state_pool.clone(),
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("boards", &boards);

    let threads = board_threads (
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
) -> Result<Uuid, Box<dyn Error>> {
    let id = create_thread (
	uid,
	&thread_form.subject,
	&thread_form.comment,
	boardname.to_string(),
	state_pool,
    ).await?;

    Ok(id)
}

pub async fn board_x_page (
    state_pool: State<PgPool>,
    Path(url): Path<String>,
) -> Result<Html<String>, axum::http::StatusCode> {
    board_page(
	state_pool,
	&url,
    ).await
}

pub async fn board_x_submission (
    state_pool: State<PgPool>,
    Path(boardname): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(thread_form): Form<ThreadForm>,
) -> impl IntoResponse {
    let uid = hashed(addr.ip());

    match is_user_suspended(uid, state_pool.clone()).await {
	Ok(true) => return Err(
	    (
		axum::http::StatusCode::FORBIDDEN,
		USER_SUSPENDED_REPLY,
	    ),
	),
	Ok(false) => (),
	Err(e) => {
	    eprintln!("Error checking user suspension: {}", e);
	    return Err(
		(
		    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	},
    }

    let number_of_posts_by_user = match number_of_pending_posts_in_last_hour(
	uid,
	state_pool.clone(),
    ).await {
	Ok(number) => number,
	Err(e) => {
	    eprintln!("Error checking user pending posts number: {}", e);
	    return Err(
		(
		    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	},
    };
    if number_of_posts_by_user > 10 {
	return Err(
	    (
		axum::http::StatusCode::TOO_MANY_REQUESTS,
		"আপনার গত এক ঘণ্টায় অনেক অনির্বাচিত পোস্ট রয়েছে। পুনরায় পোস্ট \
		 করার আগে অনুগ্রহ করে কিছুক্ষণ অপেক্ষা করুন।",
	    )
	)
    };

    match board_submission(
	state_pool,
	uid,
	thread_form,
	&boardname,
    ).await {
	Ok(id) => {
	    Ok(Redirect::to(&format!("/submission/{}", id)))
	},
	Err(e) => {
	    eprintln!("Error submitting thread: {}", e);
	    Err(
		(
		    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
		    INTERNAL_SERVER_ERROR_REPLY,
		)
	    )
	},
    }
}
