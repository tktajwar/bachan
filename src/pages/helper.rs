use std::error::Error;
use std::net::IpAddr;
use std::hash::{DefaultHasher, Hash, Hasher};
use sqlx::PgPool;

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
