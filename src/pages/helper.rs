use axum::extract::State;
use serde::Serialize;
use std::error::Error;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::net::IpAddr;
use sqlx::{
    PgPool,
    types::chrono,
    types::chrono::DateTime,
};
use uuid::Uuid;

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
    pub redacted: bool,
    pub reply_count: i32,
}

#[derive(sqlx::FromRow)]
pub struct ThreadLight {
    pub id: i32,
    pub uid: i32,
    pub subject: String,
    pub comment: String,
    pub ctime: chrono::NaiveDateTime,
    pub redacted: bool,
}

#[derive(sqlx::FromRow)]
pub struct Reply {
    pub id: i32,
    pub uid: i32,
    pub tid: i32,
    pub comment: String,
    pub ctime: chrono::NaiveDateTime,
    pub redacted: bool,
}

#[derive(sqlx::FromRow)]
pub struct ReplyLight {
    pub id: i32,
    pub uid: i32,
    pub tid: i32,
    pub comment: String,
    pub ctime: chrono::NaiveDateTime,
    pub redacted: bool,
}

#[derive(sqlx::FromRow)]
#[derive(Serialize)]
pub struct PendingPost {
    pub id: Uuid,
    pub is_thread: bool,
    pub subject: Option<String>,
    pub comment: String,
    pub board: Option<String>,
    pub tid: Option<i32>,
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
    pub reply_count: i32,
    pub redacted: bool,
}

#[derive(Serialize)]
pub struct ReplySerializable {
    pub id: String,
    pub tid: String,
    pub utid: String,
    pub comment: String,
    pub ctime: String,
}

#[derive(Serialize)]
pub struct ThreadOrReplySerializable {
    pub id: String,
    pub tid: Option<String>,
    pub utid: String,
    pub subject: String,
    pub comment: String,
    pub ctime: String,
    pub redacted: bool,
}

impl Thread {
    pub fn into_serializable(self) -> ThreadSerializable {
	ThreadSerializable {
	    id: format!("{:03x}", self.id as u32),
	    utid: utid(self.uid, self.id),
	    subject: if self.redacted {
		"স#ম্পা#দি#ত".to_string()
	    } else {
		self.subject
	    },
	    comment: if self.redacted {
		"<del>###সম্পাদিত###</del>".to_string()
	    } else {
		self.comment
	    },
	    board: self.board,
	    ctime: self.ctime.format("%Y-%m-%d %H:%MZ").to_string(),
	    mtime: self.mtime.format("%Y-%m-%d %H:%MZ").to_string(),
	    reply_count: self.reply_count,
	    redacted: self.redacted,
	}
    }
}

impl ThreadLight {
    async fn try_from(
	id: i32,
	State(pool): State<PgPool>,
    ) -> Result<ThreadLight, Box<dyn Error>> {
	let q = "\
	SELECT \
	id, \
	uid, \
	subject, \
	comment, \
	ctime, \
	redacted \
	FROM Thread \
	WHERE id = $1 \
	";

	let thread: ThreadLight = sqlx::query_as::<_, ThreadLight>(q)
	    .bind(id)
	    .fetch_one(&pool)
	    .await?;

	Ok(thread)
    }
}

impl ThreadLight {
    pub fn into_serializable(self) -> ThreadOrReplySerializable {
	ThreadOrReplySerializable {
	    id: format!("{:03x}", self.id as u32),
	    tid: None,
	    utid: utid(self.uid, self.id),
	    subject: if self.redacted {
		"স#ম্পা#দি#ত".to_string()
	    } else {
		self.subject
	    },
	    comment: if self.redacted {
		"<del>###সম্পাদিত###</del>".to_string()
	    } else {
		self.comment
	    },
	    ctime: self.ctime.format("%Y-%m-%d %H:%MZ").to_string(),
	    redacted: self.redacted,
	}
    }
}

impl Reply {
    pub fn into_serializable(self) -> ReplySerializable {
	ReplySerializable {
	    id: format!("{:03x}", self.id as u32),
	    tid: format!("{:03x}", self.tid as u32),
	    utid: utid(self.uid, self.tid),
	    comment: if self.redacted {
		"<del>###সম্পাদিত###</del>".to_string()
	    } else {
		self.comment
	    },
	    ctime: self.ctime.format("%Y-%m-%d %H:%MZ").to_string(),
	}
    }
}

impl ReplyLight {
    async fn try_from(
	id: i32,
	State(pool): State<PgPool>,
    ) -> Result<ReplyLight, Box<dyn Error>> {
	let q = "\
	SELECT \
	id, \
	uid, \
	tid, \
	comment, \
	ctime, \
	redacted \
	FROM Reply \
	WHERE id = $1 \
	";

	let reply: ReplyLight = sqlx::query_as::<_, ReplyLight>(q)
	    .bind(id)
	    .fetch_one(&pool)
	    .await?;

	Ok(reply)
    }
}

impl ReplyLight {
    pub fn into_serializable(self) -> ThreadOrReplySerializable {
	ThreadOrReplySerializable {
	    id: format!("{:03x}", self.id as u32),
	    tid: Some(format!("{:03x}", self.tid as u32)),
	    utid: utid(self.uid, self.tid),
	    subject: format!("Reply to {:03x}", self.tid).to_string(),
	    comment: if self.redacted {
		"<del>###সম্পাদিত###</del>".to_string()
	    } else {
		self.comment
	    },
	    ctime: self.ctime.format("%Y-%m-%d %H:%MZ").to_string(),
	    redacted: self.redacted,
	}
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

pub fn utid(uid: i32, tid: i32) -> String {
    let mut hasher = DefaultHasher::new();

    uid.hash(&mut hasher);
    tid.hash(&mut hasher);
    crate::SECRET_NUMBER.hash(&mut hasher);

    let utid = hasher.finish();

    let utid = format!("{:05}", utid as u16);

    bengali_digits(&utid)
}

pub fn bengali_digits(s: &str) -> String {
    s.chars().map(|c| {
	match c {
	    '0' => '০',
            '1' => '১',
            '2' => '২',
            '3' => '৩',
            '4' => '৪',
            '5' => '৫',
            '6' => '৬',
            '7' => '৭',
            '8' => '৮',
            '9' => '৯',
	    'a' => 'ক',
            'b' => 'খ',
            'c' => 'গ',
            'd' => 'ঘ',
            'e' => 'ঙ',
            'f' => 'চ',
            _ => c,
	}
    }).collect()
}

pub async fn thread_or_reply_with_id(
    id: i32,
    state_pool: State<PgPool>,
) -> Result<ThreadOrReplySerializable, Box<dyn Error>> {
    let Ok(thread) = ThreadLight::try_from(id, state_pool.clone()).await else {
	let reply: ReplyLight = ReplyLight::try_from(id, state_pool).await?;
	let serialize_reply = reply.into_serializable();
	return Ok(serialize_reply)
    };
    let serializable_thread = thread.into_serializable();
    Ok(serializable_thread)
}

pub async fn create_thread (
    uid: i32,
    subject: &str,
    comment: &str,
    board: String,
    State(pool): State<PgPool>,
) -> Result<Uuid, Box<dyn Error>> {
    let comment_formatted = formatting::format(comment);

    let query = "\
    INSERT INTO PendingPost (is_thread, uid, subject, comment, board) \
    VALUES (TRUE, $1, $2, $3, $4) \
    RETURNING id \
    ";


    let id: Uuid = sqlx::query_scalar(query)
	.bind(uid)
	.bind(subject)
	.bind(comment_formatted)
	.bind(board)
	.fetch_one(&pool)
	.await?;

    Ok(id)
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
    ctime, \
    redacted
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

pub async fn create_reply (
    uid: i32,
    tid: i32,
    comment: &str,
    State(pool): State<PgPool>,
) -> Result<Uuid, Box<dyn Error>> {
    let comment_formatted = formatting::format(comment);

    let query = "\
    INSERT INTO PendingPost (is_thread, uid, tid, comment) \
    VALUES (False, $1, $2, $3) \
    RETURNING id \
    ";

    let id: Uuid = sqlx::query_scalar(query)
	.bind(uid)
	.bind(tid)
	.bind(comment_formatted)
	.fetch_one(&pool)
	.await?;

    Ok(id)
}

pub async fn redact_thread_or_reply(
    id: i32,
    moderator_username: &str,
    reason: &str,
    State(pool): State<PgPool>,
) -> Result<(), Box<dyn Error>> {
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

    sqlx::query(
	"\
	INSERT INTO redacted(thread_or_reply_id, mod_id, reason) \
	VALUES ($1, $2, $3) \
	"
    )
	.bind(id)
	.bind(moderator_id)
	.bind(reason)
	.execute(&pool)
	    .await?;

    sqlx::query(
	"\
	UPDATE thread \
	SET redacted = true \
	WHERE id = $1 \
	"
    )
	.bind(id)
	.execute(&pool)
	.await?;
    sqlx::query(
	"\
	UPDATE reply \
	SET redacted = true \
	WHERE id = $1 \
	"
    )
	.bind(id)
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
    State(pool): State<PgPool>,
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

pub async fn top_announcements (
    State(pool): State<PgPool>,
) -> Result<Vec<ThreadSerializable>, Box<dyn Error>> {
    let q = "\
    SELECT \
    id, \
    uid, \
    subject, \
    comment, \
    board, \
    ctime, \
    mtime, \
    redacted, \
    reply_count \
    FROM thread \
    WHERE redacted = false \
    AND board = 'g' \
    AND mtime > NOW() - interval'7 day'
    ORDER BY id desc \
    LIMIT 3 \
    ";

    let threads = sqlx::query_as::<_, Thread>(q)
	.fetch_all(&pool)
	.await?;

    let serializable_threads: Vec<ThreadSerializable> = threads.into_iter()
        .map(Thread::into_serializable)
        .collect();

    Ok(serializable_threads)
}

pub async fn top_threads (
    limit: i32,
    State(pool): State<PgPool>,
) -> Result<Vec<ThreadSerializable>, Box<dyn Error>> {
    let q = "\
    SELECT \
    id, \
    uid, \
    subject, \
    comment, \
    board, \
    ctime, \
    mtime, \
    redacted, \
    reply_count \
    FROM thread \
    WHERE redacted = false \
    AND board <> 'g' \
    AND ( \
    mtime > NOW() - interval'6 hour' \
    OR \
    ( mtime > NOW() - interval'7 day' AND reply_count > 3 ) \
    ) \
    ORDER BY mtime desc \
    LIMIT $1 \
    ";

    let threads = sqlx::query_as::<_, Thread>(q)
	.bind(limit)
	.fetch_all(&pool)
	.await?;

    let serializable_threads: Vec<ThreadSerializable> = threads.into_iter()
        .map(Thread::into_serializable)
        .collect();

    Ok(serializable_threads)
}

pub async fn list_of_boards (
    State(pool): State<PgPool>,
) -> Result<Vec<Board>, Box<dyn Error>> {
    let q = "\
    SELECT \
    url, \
    label \
    FROM board \
    ORDER BY url ASC \
    ";

    let boards: Vec<Board> = sqlx::query_as::<_, Board>(q)
	.fetch_all(&pool)
	.await?;

    Ok(boards)
}

pub async fn pending_post_with_id (
    id: Uuid,
    State(pool): State<PgPool>,
) -> Result<Option<PendingPost>, Box<dyn Error>> {
    let q = "
    SELECT \
    id, \
    is_thread, \
    subject, \
    comment, \
    board, \
    tid \
    FROM PendingPost \
    WHERE id = $1 \
    ";

    let pending_post: Option<PendingPost> = sqlx::query_as::<_, PendingPost>(q)
	.bind(id)
	.fetch_optional(&pool)
	.await?;

    Ok(pending_post)
}

pub async fn confirm_post (
    id: Uuid,
    uid: i32,
    state_pool: State<PgPool>,
) -> Result<Option<i32>, Box<dyn Error>> {
    let Some(pending_post) = pending_post_with_id(
	id,
	state_pool.clone(),
    ).await? else {
	return Ok(None)
    };

    let pending_post_id = pending_post.id;

    let posted_id = if pending_post.is_thread {
	confirm_thread(
	    pending_post,
	    uid,
	    state_pool.clone(),
	).await?
    } else {
	confirm_reply(
	    pending_post,
	    uid,
	    state_pool.clone(),
	).await?
    };

    delete_pending_post(
	pending_post_id,
	state_pool,
    ).await?;

    Ok(Some(posted_id))
}

async fn confirm_thread (
    pending_post: PendingPost,
    uid: i32,
    State(pool): State<PgPool>,
) -> Result<i32, Box<dyn Error>> {
    let query = "\
    INSERT INTO thread (uid, subject, comment, board) \
    VALUES ($1, $2, $3, $4) \
    RETURNING id \
    ";

    let id: i32 = sqlx::query_scalar(query)
	.bind(uid)
	.bind(pending_post.subject)
	.bind(pending_post.comment)
	.bind(pending_post.board)
	.fetch_one(&pool)
	.await?;

    Ok(id)
}

async fn confirm_reply (
    pending_post: PendingPost,
    uid: i32,
    State(pool): State<PgPool>,
) -> Result<i32, Box<dyn Error>> {
    let query = "\
    INSERT INTO reply (uid, tid, comment) \
    VALUES ($1, $2, $3) \
    RETURNING id \
    ";

    let id: i32 = sqlx::query_scalar(query)
	.bind(uid)
	.bind(pending_post.tid)
	.bind(pending_post.comment)
	.fetch_one(&pool)
	.await?;

    Ok(id)
}

pub async fn delete_pending_post (
    id: Uuid,
    State(pool): State<PgPool>,
) -> Result<(), Box<dyn Error>> {
    let q = "\
    DELETE FROM PendingPost \
    WHERE id = $1 \
    ";

    sqlx::query(q)
	.bind(id)
	.execute(&pool)
	.await?;

    Ok(())
}

pub async fn number_of_pending_posts_in_last_hour (
    uid: i32,
    State(pool): State<PgPool>,
) -> Result<i64, Box<dyn Error>> {
    let q = "
    SELECT COUNT(1) \
    FROM PendingPost \
    WHERE uid = $1 \
    AND ctime > NOW() - interval'1 hour' \
    ";

    let (number,): (i64,) = sqlx::query_as::<_, (i64,)>(q)
	.bind(uid)
	.fetch_one(&pool)
	.await?;

    Ok(number)
}

pub async fn number_of_threads_in_last_hour (
    uid: i32,
    State(pool): State<PgPool>,
) -> Result<i64, Box<dyn Error>> {
    let q = "
    SELECT COUNT(1) \
    FROM thread \
    WHERE uid = $1 \
    AND ctime > NOW() - interval'1 hour' \
    ";

    let (number,): (i64,) = sqlx::query_as::<_, (i64,)>(q)
	.bind(uid)
	.fetch_one(&pool)
	.await?;

    Ok(number)
}

pub async fn number_of_replies_in_last_hour (
    uid: i32,
    State(pool): State<PgPool>,
) -> Result<i64, Box<dyn Error>> {
    let q = "
    SELECT COUNT(1) \
    FROM reply \
    WHERE uid = $1 \
    AND ctime > NOW() - interval'1 hour' \
    ";

    let (number,): (i64,) = sqlx::query_as::<_, (i64,)>(q)
	.bind(uid)
	.fetch_one(&pool)
	.await?;

    Ok(number)
}

pub async fn paginated_threads (
    before_id_optional: Option<i32>,
    limit_opt: Option<i32>,
    State(pool): State<PgPool>,
) -> Result<(Vec<ThreadSerializable>, bool, i32), Box<dyn Error>> {
    let limit = match limit_opt {
	Some(limit) => limit,
	None => 20,
    };

    let threads: Vec<Thread> = {
	if let Some(before_id) = before_id_optional {
	    let q = "\
	    SELECT \
	    id, \
	    uid, \
	    subject, \
	    comment, \
	    board, \
	    ctime, \
	    mtime, \
	    redacted, \
	    reply_count \
	    FROM thread \
	    WHERE id < $1 \
	    ORDER BY id desc \
	    LIMIT $2 \
	    ";

	    sqlx::query_as::<_, Thread>(q)
		.bind(before_id)
		.bind(limit+1)
		.fetch_all(&pool)
		.await?
	} else {
	    let q = "\
	    SELECT \
	    id, \
	    uid, \
	    subject, \
	    comment, \
	    board, \
	    ctime, \
	    mtime, \
	    redacted, \
	    reply_count \
	    FROM thread \
	    ORDER BY id desc \
	    LIMIT $1 \
	    ";

	    sqlx::query_as::<_, Thread>(q)
		.bind(limit+1)
		.fetch_all(&pool)
		.await?
	}
    };

    let mut serializable_threads: Vec<ThreadSerializable> = threads.into_iter()
        .map(Thread::into_serializable)
        .collect();

    let has_more = serializable_threads.len() > limit as usize;
    if has_more {
	serializable_threads.pop();
    }

    Ok((serializable_threads, has_more, limit))
}

pub async fn paginated_board_threads (
    before_mtime_optional: Option<i64>,
    limit_opt: Option<i32>,
    board: &str,
    State(pool): State<PgPool>,
) -> Result<(Vec<ThreadSerializable>, bool, i32, i64), Box<dyn Error>> {
    let limit = match limit_opt {
	Some(limit) => limit,
	None => 20,
    };

    let mut threads = if let Some(before_mtime) = before_mtime_optional {
	let q = "\
	SELECT \
	id, \
	uid, \
	subject, \
	comment, \
	board, \
	ctime, \
	mtime, \
	redacted, \
	reply_count \
	FROM thread \
	WHERE board = $1 \
	AND redacted = false \
	AND mtime < $2
	ORDER BY mtime desc \
	LIMIT $3 \
	";

	sqlx::query_as::<_, Thread>(q)
	    .bind(board)
	    .bind(DateTime::from_timestamp(before_mtime, 0))
	    .bind(limit+1)
	    .fetch_all(&pool)
	    .await?
    } else {
	let q = "\
	SELECT \
	id, \
	uid, \
	subject, \
	comment, \
	board, \
	ctime, \
	mtime, \
	redacted, \
	reply_count \
	FROM thread \
	WHERE board = $1 \
	AND redacted = false \
	ORDER BY mtime desc \
	LIMIT $2 \
	";

	sqlx::query_as::<_, Thread>(q)
	    .bind(board)
	    .bind(limit+1)
	    .fetch_all(&pool)
	    .await?
    };

    let has_more = threads.len() > limit as usize;
    if has_more {
	threads.pop();
    }

    let last_mtime = if threads.len() > 0 {
	threads.last().unwrap().mtime.and_utc().timestamp()
    } else {
	0
    };

    let serializable_threads: Vec<ThreadSerializable> = threads.into_iter()
        .map(Thread::into_serializable)
        .collect();

    Ok((serializable_threads, has_more, limit, last_mtime))
}
