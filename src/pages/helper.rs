use serde::Serialize;
use std::error::Error;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::net::IpAddr;
use sqlx::{
    PgPool,
    types::chrono,
};

#[derive(sqlx::FromRow)]
pub struct Thread {
    pub id: i32,
    pub uid: i32,
    pub subject: String,
    pub comment: String,
    pub board: String,
    pub ctime: chrono::NaiveDateTime,
    pub mtime: chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct ThreadSerializable {
    pub id: u32,
    pub utid: u16,
    pub subject: String,
    pub comment: String,
    pub board: String,
    pub ctime: String,
    pub mtime: String,
}

impl Thread {
    pub fn into_serializable(self) -> ThreadSerializable {
	ThreadSerializable {
	    id: self.id as u32,
	    utid: self.hashed_utid(),
	    subject: self.subject,
	    comment: self.comment,
	    board: self.board,
	    ctime: self.ctime.format("%Y-%m-%d %H:%M").to_string(),
	    mtime: self.mtime.format("%Y-%m-%d %H:%M").to_string(),
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

pub fn hashed(ip: IpAddr) -> i32 {
    let mut hasher = DefaultHasher::new();

    ip.hash(&mut hasher);
    crate::SECRET_NUMBER.hash(&mut hasher);

    hasher.finish() as i32
}

pub async fn create_thread(
    ip: IpAddr,
    subject: String,
    comment: String,
    board: String,
    pool: PgPool,
) -> Result<(), Box<dyn Error>> {
    let query = "\
    INSERT INTO thread (uid, subject, comment, board) \
    VALUES ($1, $2, $3, $4) \
    ";

    sqlx::query(query)
	.bind(hashed(ip))
	.bind(subject)
	.bind(comment)
	.bind(board)
	.execute(&pool)
	.await?;

    Ok(())
}

pub async fn board_threads(
    board: &str,
    pool: PgPool,
) -> Result<Vec<ThreadSerializable>, Box<dyn Error>> {
    let q = "\
    SELECT id, uid, subject, comment, board, ctime, mtime FROM thread \
    WHERE board = $1 \
    ORDER BY id desc \
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
