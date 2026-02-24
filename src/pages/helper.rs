use axum::extract::State;
use serde::Serialize;
use std::error::Error;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::net::IpAddr;
use sqlx::{
    PgPool,
    types::chrono,
};

use crate::formatting;

#[derive(sqlx::FromRow)]
pub struct Thread {
    pub id: i32,
    pub uid: i32,
    pub subject: String,
    pub comment: String,
    pub board: String,
    pub ctime: chrono::NaiveDateTime,
    pub mtime: chrono::NaiveDateTime,
    pub reply_count: i64,
}

#[derive(sqlx::FromRow)]
pub struct Reply {
    pub id: i32,
    pub uid: i32,
    pub tid: i32,
    pub comment: String,
    pub ctime: chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct ThreadSerializable {
    pub id: String,
    pub utid: String,
    pub subject: String,
    pub comment: String,
    pub board: String,
    pub ctime: String,
    pub mtime: String,
    pub reply_count: i64,
}

#[derive(Serialize)]
pub struct ReplySerializable {
    pub id: String,
    pub utid: String,
    pub comment: String,
    pub ctime: String,
}

impl Thread {
    pub fn into_serializable(self) -> ThreadSerializable {
	ThreadSerializable {
	    id: format!("{:03x}", self.id as u32),
	    utid: format!("{:04x}", self.hashed_utid()),
	    subject: self.subject,
	    comment: self.comment,
	    board: self.board,
	    ctime: self.ctime.format("%Y-%m-%d %H:%M").to_string(),
	    mtime: self.mtime.format("%Y-%m-%d %H:%M").to_string(),
	    reply_count: self.reply_count,
	}
    }

    pub fn hashed_utid(&self) -> u16 {
	let mut hasher = DefaultHasher::new();

	self.id.hash(&mut hasher);
	self.uid.hash(&mut hasher);
	crate::SECRET_NUMBER.hash(&mut hasher);

	hasher.finish() as u16
    }
}

impl Reply {
    pub fn into_serializable(self) -> ReplySerializable {
	ReplySerializable {
	    id: format!("{:03x}", self.id as u32),
	    utid: format!("{:04x}", self.hashed_utid()),
	    comment: self.comment,
	    ctime: self.ctime.format("%Y-%m-%d %H:%M").to_string(),
	}
    }

    pub fn hashed_utid(&self) -> u16 {
	let mut hasher = DefaultHasher::new();

	self.tid.hash(&mut hasher);
	self.uid.hash(&mut hasher);
	crate::SECRET_NUMBER.hash(&mut hasher);

	hasher.finish() as u16
    }
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Board {
    pub url: String,
    pub label: String,
}

pub fn hashed(ip: IpAddr) -> i32 {
    let mut hasher = DefaultHasher::new();

    ip.hash(&mut hasher);
    crate::SECRET_NUMBER.hash(&mut hasher);

    hasher.finish() as i32
}

pub async fn create_thread(
    ip: IpAddr,
    subject: &str,
    comment: &str,
    board: String,
    pool: PgPool,
) -> Result<(), Box<dyn Error>> {
    let comment_formatted = formatting::format(comment);

    let query = "\
    INSERT INTO thread (uid, subject, comment, board) \
    VALUES ($1, $2, $3, $4) \
    ";

    sqlx::query(query)
	.bind(hashed(ip))
	.bind(subject)
	.bind(comment_formatted)
	.bind(board)
	.execute(&pool)
	.await?;

    Ok(())
}

pub async fn thread_replies(
    tid: i32,
    pool: PgPool,
) -> Result<Vec<ReplySerializable>, Box<dyn Error>> {
    let q = "\
    SELECT \
    id, \
    uid, \
    tid, \
    comment, \
    ctime \
    FROM reply \
    WHERE tid = $1 \
    ORDER BY id ASC \
    ";

    let replies = sqlx::query_as::<_, Reply>(q)
	.bind(tid)
	.fetch_all(&pool)
	.await?;

    let serializable_replies: Vec<ReplySerializable> = replies.into_iter()
        .map(Reply::into_serializable)
        .collect();

    Ok(serializable_replies)
}

pub async fn create_reply(
    ip: IpAddr,
    tid: i32,
    comment: &str,
    pool: PgPool,
) -> Result<(), Box<dyn Error>> {
    let comment_formatted = formatting::format(comment);

    let query = "\
    INSERT INTO reply (uid, tid, comment) \
    VALUES ($1, $2, $3) \
    ";

    sqlx::query(query)
	.bind(hashed(ip))
	.bind(tid)
	.bind(comment_formatted)
	.execute(&pool)
	.await?;

    Ok(())
}

pub async fn boards_in_category(
    category: &str,
    State(pool): State<PgPool>,
) -> Result<Vec<Board>, Box<dyn Error>> {
    let q = "\
    SELECT \
    url, \
    label \
    FROM board \
    WHERE category = $1 \
    ORDER BY url ASC \
    ";

    let boards: Vec<Board> = sqlx::query_as::<_, Board>(q)
	.bind(category)
	.fetch_all(&pool)
	.await?;

    Ok(boards)
}

pub async fn get_board_ctx(
    url: &str,
    pool: PgPool,
) -> Result<Board, Box<dyn Error>> {
    let q = "\
    SELECT \
    url, \
    label \
    FROM board \
    WHERE url = $1 \
    ORDER BY url ASC \
    ";

    let board: Board = sqlx::query_as::<_, Board>(q)
	.bind(url)
	.fetch_one(&pool)
	.await?;

    Ok(board)
}
