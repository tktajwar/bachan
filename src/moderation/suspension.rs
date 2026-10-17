use sqlx::{
    PgPool,
    types::chrono::Utc,
};
use std::error::Error;
use std::net::IpAddr;
use std::time::Duration;

pub async fn suspend_user (
    post_id: i64,
    moderator_username: &str,
    for_days: u64,
    reason: &str,
    pool: PgPool,
    suspended_cache: crate::SuspendedCache,
) -> Result<(), Box<dyn Error>> {
    let user_id_opt = sqlx::query_scalar::<_, Option<IpAddr>>(
	"\
	SELECT user_id \
	FROM Post \
	WHERE id = $1 \
	"
    )
	.bind(post_id)
	.fetch_one(&pool)
	.await?;

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

    if let Some(user_id) = user_id_opt {
	sqlx::query (
	    "\
	    INSERT INTO suspended(user_id, mod_id, post, until, reason) \
	    VALUES ($1, $2, $3, $4, $5) \
	    "
	)
	    .bind(user_id)
	    .bind(moderator_id)
	    .bind(post_id)
	    .bind(suspend_until)
	    .bind(reason)
	    .execute(&pool)
	    .await?;

	suspended_cache.invalidate(&()).await;
    }

    Ok(())
}

pub async fn is_user_suspended (
    user_id: IpAddr,
    pool: PgPool,
    suspended_cache: crate::SuspendedCache,
) -> Result<bool, Box<dyn Error>> {
    if let Some(suspended_ips) = suspended_cache.get(&()).await {
	return Ok(suspended_ips.contains(&user_id))
    };

    let suspended_ips = sqlx::query_scalar::<_, IpAddr>(
	"\
	SELECT user_id \
	FROM Suspended \
	WHERE user_id IS NOT NULL \
	AND until > NOW() \
	"
    )
	.bind(user_id)
	.fetch_all(&pool)
	.await?;

    eprintln!("Loading suspended users table into memory");

    let suspended_ips = suspended_ips
	.into_iter()
	.collect::<crate::HashSet<IpAddr>>();

    suspended_cache.insert(
	(),
	suspended_ips.clone(),
    ).await;

    Ok(suspended_ips.contains(&user_id))
}
