use sqlx::{
    PgPool,
    types::chrono::Utc,
};
use std::error::Error;
use std::time::Duration;

pub async fn suspend_user (
    post_id: i64,
    moderator_username: &str,
    for_days: u64,
    reason: &str,
    pool: PgPool,
) -> Result<(), Box<dyn Error>> {
    let (uid,) = match sqlx::query_as::<_, (i64,)>(
	"\
	SELECT user_id \
	FROM Post \
	WHERE id = $1 \
	"
    )
	.bind(post_id)
	.fetch_one(&pool)
	.await {
	    Ok(uid) => uid,
	    Err(_) => {
		sqlx::query_as::<_, (i64,)>(
		    "\
		    SELECT uid \
		    FROM  reply \
		    WHERE id = $1 \
		    "
		)
		    .bind(post_id)
		    .fetch_one(&pool)
		    .await?
	    },
    };

    let (moderator_id,): (i32,) = sqlx::query_as::<_, (i32,)>(
	"\
	SELECT id \
	FROM mod \
	WHERE username = $1 \
	"
    )
	.bind(moderator_username)
	.fetch_one(&pool)
	.await?;

    let suspend_until = Utc::now() + Duration::from_secs(
	for_days * 24 * 60 * 60
    );

    sqlx::query(
	"\
	INSERT INTO suspended(uid, mod_id, post, until, reason) \
	VALUES ($1, $2, $3, $4, $5) \
	"
    )
	.bind(uid)
	.bind(moderator_id)
	.bind(post_id)
	.bind(suspend_until)
	.bind(reason)
	.execute(&pool)
	.await?;

    Ok(())
}

pub async fn is_user_suspended (
    uid: i32,
    pool: PgPool,
) -> Result<bool, Box<dyn Error>> {
    let (is_suspended,): (bool,) = sqlx::query_as::<_, (bool,)>(
	"\
	SELECT EXISTS ( \
	  SELECT 1 \
	  FROM Suspended \
	  WHERE uid = $1 \
	  AND until > NOW() \
	) \
	"
    )
	.bind(uid)
	.fetch_one(&pool)
	.await?;

    Ok(is_suspended)
}
