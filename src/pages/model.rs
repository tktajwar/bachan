use base64::{
    engine::general_purpose::URL_SAFE_NO_PAD,
    Engine as _,
};
use maxminddb::{Reader, PathElement};
use serde::Deserialize;
use serde::Serialize;
use siphasher::sip::SipHasher13;
use std::collections::HashMap;
use std::error::Error;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::net::IpAddr;
use sqlx::{
    PgPool,
    types::chrono,
    types::chrono::DateTime,
};

use crate::sanitize::sanitize;

use crate::forms::ReplyForm;

#[derive(sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub board: Option<String>,
    pub thread_id: Option<i64>,

    pub subject: Option<String>,
    pub comment: String,
    pub user_id: i64,
    pub user_trip: Option<i64>,
    pub user_country: String,
    pub user_name: String,

    pub birth_time: chrono::DateTime<chrono::Utc>,
    pub change_time: Option<chrono::DateTime<chrono::Utc>>,
    pub bump_time: Option<chrono::DateTime<chrono::Utc>>,

    pub reply_count: Option<i32>,
    pub is_redacted: bool,
    pub is_saged: bool,
}

#[derive(Serialize,Clone)]
pub struct PostSerialized {
    pub id: String,
    pub board: Option<String>,
    pub thread_id: Option<String>,

    pub subject: Option<String>,
    pub comment: String,
    pub user_code: String,
    pub user_country: String,
    pub user_name: String,

    pub birth_time: chrono::DateTime<chrono::Utc>,
    pub change_time: Option<chrono::DateTime<chrono::Utc>>,

    pub reply_count: Option<i32>,
    pub is_redacted: bool,
    pub is_saged: bool,
}

#[derive(Serialize)]
pub struct Thread {
    pub id: String,
    pub board: Option<String>,

    pub subject: Option<String>,
    pub comment: String,
    pub user_code: String,
    pub user_country: String,
    pub user_name: String,

    pub birth_time: chrono::DateTime<chrono::Utc>,
    pub change_time: Option<chrono::DateTime<chrono::Utc>>,

    pub reply_count: Option<i32>,
    pub is_redacted: bool,
    pub is_saged: bool,

    pub replies: Vec<PostSerialized>,
}

#[derive(Serialize)]
pub struct Threads {
    pub threads: Vec<Thread>,
}

#[derive(sqlx::FromRow)]
#[derive(Serialize)]
pub struct TopThread {
    pub id: String,
    pub subject: String,
    pub comment: String,
    pub board: String,
    pub label: String,
    pub reply_count: i32,
    pub bump_time: Option<chrono::DateTime<chrono::Utc>>,
}

fn decode_user_name (
    code_opt: &Option<String>
) -> (String, Option<i64>) {
    let Some(code) = code_opt else { return ("বানন".to_string(), None) };

    let mut split = code.split('#');

    let user_name = split.next().map(sanitize)
	.filter(|s| s.len() > 0)
	.map(|s|
	     if s.len() > 32 {
		 s[..32].to_string()
	     } else {
		s.to_string()
	     }
	)
	.unwrap_or("বানন".to_string());

    let user_trip = split.next()
	.filter(|s| s.len() > 0)
	.map(sanitize)
	.map(trip);

    (user_name, user_trip)
}

#[derive(Debug)]
pub struct HexID(pub i64);

impl<'de> Deserialize<'de> for HexID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;

        i64::from_str_radix(&id, 16)
            .map(HexID)
            .map_err(|_| serde::de::Error::custom("invalid hexadecimal ID"))
    }
}

impl Post {
    pub fn user_code(&self) -> String {
	let mut hasher = DefaultHasher::new();

	if let Some(code) = self.user_trip {
	    code.hash(&mut hasher);
	    let hashed = hasher.finish() as u32;

	    return URL_SAFE_NO_PAD.encode(hashed.to_be_bytes())
	}

	let tid = match self.thread_id {
	    Some(tid) => tid,
	    None => self.id,
	};

	self.user_id.hash(&mut hasher);
	tid.hash(&mut hasher);

	let hashed = hasher.finish();

	let user_code = format!("{:08x}", hashed as u32);

	bengali_digits(&user_code)
    }
}

impl From<Post> for PostSerialized {
    fn from(post: Post) -> Self {
	PostSerialized {
	    user_code: post.user_code(),

	    id: format!("{:03x}", post.id),
	    board: post.board,
	    thread_id: post.thread_id.map(|tid| format!("{:03x}", tid)),

	    subject: post.subject,
	    comment: post.comment,
	    user_country: post.user_country,
	    user_name: post.user_name,

	    birth_time: post.birth_time,
	    change_time: post.change_time,

	    reply_count: post.reply_count,
	    is_redacted: post.is_redacted,
	    is_saged: post.is_saged,
	}
    }
}

impl From<(Vec<Post>, Vec<Post>)> for Threads {
    fn from (
	posts: (Vec<Post>, Vec<Post>)
    ) -> Self {

	let (threads, replies) = posts;

	let r_by_t: HashMap<i64, Vec<PostSerialized>> = replies
	    .into_iter()
	    .fold(HashMap::new(), |mut map, reply| {
		let thread_id = reply.thread_id.unwrap();
		map.entry(thread_id).or_insert_with(Vec::new).push(
		    PostSerialized::from(reply)
		);
		map
	    });

	Threads { threads: threads
		  .into_iter()
		  .map(|thread| Thread {
		      user_code: thread.user_code(),

		      id: format!("{:03x}", thread.id),
		      board: thread.board,

		      subject: thread.subject,
		      comment: thread.comment,
		      user_country: thread.user_country,
		      user_name: thread.user_name,

		      birth_time: thread.birth_time,
		      change_time: thread.change_time,

		      reply_count: thread.reply_count,
		      is_redacted: thread.is_redacted,
		      is_saged: thread.is_saged,

		      replies: r_by_t.get(&thread.id).cloned().unwrap_or_default(),
		  })
		  .collect()
	}

    }
}

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct Board {
    pub url: String,
    pub label: String,
}

pub fn trip(pass: String) -> i64 {
    let mut hasher = SipHasher13::new_with_keys(
	0x0123_4567_89ab_cdef,
	0xfedc_ba98_7654_3210,
    );
    pass.to_lowercase().hash(&mut hasher);
    hasher.finish() as i64
}

pub fn hashed(ip: IpAddr) -> i64 {
    let mut hasher = SipHasher13::new_with_keys(
	0x0123_4567_89ab_cdef,
	0xfedc_ba98_7654_3210,
    );
    ip.hash(&mut hasher);
    hasher.finish() as i64
}

pub fn utid (
    uid: i64,
    tid: i32,
    trip: Option<i64>,
) -> String {
    let mut hasher = DefaultHasher::new();

    if let Some(code) = trip {
	code.hash(&mut hasher);
	let hashed = hasher.finish() as u32;

	return URL_SAFE_NO_PAD.encode(hashed.to_be_bytes())
    }

    uid.hash(&mut hasher);
    tid.hash(&mut hasher);

    let utid = hasher.finish();

    let utid = format!("{:08x}", utid as u32);

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

pub async fn create_thread (
    ip: IpAddr,
    subject: &str,
    comment: &str,
    name: Option<String>,
    board: &str,
    pool: PgPool,
    pages_cache: crate::PagesCache,
) -> Result<i64, Box<dyn Error>> {
    let q = "\
    INSERT INTO Post ( \
      board, subject, comment, user_id, user_trip, user_country, user_name, \
      change_time, bump_time, reply_count \
    ) \
    VALUES ( \
      $1, $2, $3, $4, $5, $6, $7, \
      NOW(), NOW(), 0 \
    ) \
    RETURNING id \
    ";

    let user_id = hashed(ip);
    let user_country = country_code(ip)?;
    let (user_name, user_trip) = decode_user_name(&name);

    let id: i64 = sqlx::query_scalar(q)
	.bind(board)
	.bind(subject.trim())
	.bind(comment.trim())
	.bind(user_id)
	.bind(user_trip)
	.bind(user_country)
	.bind(user_name)
	.fetch_one(&pool)
	.await?;

    pages_cache.invalidate("root").await;

    Ok(id)
}

pub async fn thread_replies (
    thread_id: i64,
    pool: PgPool,
) -> Result<(Vec<Post>, i64), Box<dyn Error>> {
    let q = "\
    SELECT \
    id, \
    board, \
    thread_id, \
    subject, \
    comment, \
    user_id, \
    user_trip, \
    user_country, \
    user_name, \
    birth_time, \
    change_time, \
    bump_time, \
    reply_count, \
    is_redacted, \
    is_saged \
    FROM Post \
    WHERE thread_id = $1 \
    ORDER BY id \
    ";

    let replies = sqlx::query_as::<_, Post>(q)
	.bind(thread_id)
	.fetch_all(&pool)
	.await?;

    let last_id = match replies.last() {
	Some(post) => post.id,
	None => 0,
    };

    Ok((replies, last_id))
}

pub async fn thread_replies_after (
    thread_id: i64,
    after_id: i64,
    pool: PgPool,
) -> Result<(Vec<Post>, i64), Box<dyn Error>> {
    
    let q = "\
    SELECT \
    id, \
    board, \
    thread_id, \
    subject, \
    comment, \
    user_id, \
    user_trip, \
    user_country, \
    user_name, \
    birth_time, \
    change_time, \
    bump_time, \
    reply_count, \
    is_redacted, \
    is_saged \
    FROM Post \
    WHERE thread_id = $1 \
    AND id > $2
    ORDER BY id \
    ";

    let replies = sqlx::query_as::<_, Post>(q)
	.bind(thread_id)
	.bind(after_id)
	.fetch_all(&pool)
	.await?;

    let last_id = if replies.len() > 0 {
	replies.last().unwrap().id
    } else {
	after_id
    };

    Ok((replies, last_id))
}

pub async fn create_reply (
    ip: IpAddr,
    thread_id: i64,
    reply_form: ReplyForm,
    pool: PgPool,
    pages_cache: crate::PagesCache,
) -> Result<i64, Box<dyn Error>> {
    let q = "\
    INSERT INTO Post ( \
      thread_id, comment, user_id, user_trip, user_country, user_name \
    ) \
    VALUES ( \
      $1, $2, $3, $4, $5, $6 \
    ) \
    RETURNING id \
    ";

    let ReplyForm { comment, name } = reply_form;
    let user_id = hashed(ip);
    let user_country = country_code(ip)?;
    let (user_name, user_trip) = decode_user_name(&name);

    let id: i64 = sqlx::query_scalar(q)
	.bind(thread_id)
	.bind(comment.trim())
	.bind(user_id)
	.bind(user_trip)
	.bind(user_country)
	.bind(user_name)
	.fetch_one(&pool)
	.await?;

    pages_cache.invalidate("root").await;

    Ok(id)
}

pub async fn redact_post (
    id: i64,
    moderator_username: &str,
    reason: &str,
    pool: PgPool,
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
	INSERT INTO RedactedPost(post, moderator, reason) \
	VALUES ($1, $2, $3) \
	"
    )
	.bind(id)
	.bind(moderator_id)
	.bind(reason)
	.execute(&pool)
	.await?;

    Ok(())
}

pub async fn restore_post (
    id: i64,
    pool: PgPool,
) -> Result<(), Box<dyn Error>> {
    sqlx::query(
	"\
	DELETE FROM RedactedPost \
	WHERE post = $1 \
	"
    )
	.bind(id)
	.execute(&pool)
	.await?;

    Ok(())
}

pub async fn boards_in_category(
    category: &str,
    pool: PgPool,
    boards_cache: crate::BoardsCache,
) -> Result<Vec<Board>, Box<dyn Error>> {
    if let Some(boards) = boards_cache.get(category).await {
	return Ok( boards );
    }

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

    boards_cache
	.insert(category.to_string(), boards.clone() )
	.await;

    Ok ( boards )

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

pub async fn top_threads (
    limit: i32,
    pool: PgPool,
) -> Result<(Vec<TopThread>, i64), Box<dyn Error>> {
    let q = "\
    SELECT \
    to_hex(t.id) as id, \
    t.subject, \
    t.comment, \
    t.reply_count, \
    t.bump_time, \
    t.board, \
    b.label, \
    t.reply_count \
    FROM Post t \
    INNER JOIN board b \
    ON t.board = b.url \
    WHERE t.is_redacted IS FALSE \
    AND t.bump_time > NOW() - interval'6 week' \
    ORDER BY t.bump_time desc \
    LIMIT $1 \
    ";

    let threads = sqlx::query_as::<_, TopThread>(q)
	.bind(limit)
	.fetch_all(&pool)
	.await?;

    let last_bump_time = threads
	.first()
	.and_then(|thread| thread.bump_time.as_ref())
	.map(|time|time.timestamp())
	.map(|time| time + 1)
	.unwrap_or(0);

    Ok((threads, last_bump_time))
}

pub async fn paginated_board_threads (
    opt_before_bump_time: Option<i64>,
    opt_limit: Option<i32>,
    board: &str,
    pool: PgPool,
) -> Result<(Vec<Thread>, bool, i32, i64), Box<dyn Error>> {
    let limit = match opt_limit {
	Some(limit) => limit,
	None => 20,
    };

    let mut threads = if let Some(before_bump_time) = opt_before_bump_time {
	let q = "\
	SELECT \
	id, \
	board, \
	thread_id, \
	subject, \
	comment, \
	user_id, \
	user_trip, \
	user_country, \
	user_name, \
	birth_time, \
	change_time, \
	bump_time, \
	reply_count, \
	is_redacted, \
	is_saged \
	FROM Post \
	WHERE board = $1 \
	AND is_redacted IS false \
	AND bump_time < $2
	ORDER BY bump_time desc \
	LIMIT $3 \
	";

	sqlx::query_as::<_, Post>(q)
	    .bind(board)
	    .bind(DateTime::from_timestamp(before_bump_time, 0))
	    .bind(limit+1)
	    .fetch_all(&pool)
	    .await?
    } else {
	let q = "\
	SELECT \
	id, \
	board, \
	thread_id, \
	subject, \
	comment, \
	user_id, \
	user_trip, \
	user_country, \
	user_name, \
	birth_time, \
	change_time, \
	bump_time, \
	reply_count, \
	is_redacted, \
	is_saged \
	FROM Post \
	WHERE board = $1 \
	AND is_redacted IS false \
	ORDER BY bump_time desc \
	LIMIT $2 \
	";

	sqlx::query_as::<_, Post>(q)
	    .bind(board)
	    .bind(limit+1)
	    .fetch_all(&pool)
	    .await?
    };

    let has_more = threads.len() > limit as usize;
    if has_more {
	threads.pop();
    }

    let last_bump_time = threads
	.last()
	.and_then(|thread| thread.bump_time.as_ref())
	.map(|time| time.timestamp())
	.unwrap_or(0);

    let thread_ids: Vec<i64> = threads.iter().map(|t| t.id).collect();

    let q = "\
    SELECT \
    id, \
    board, \
    thread_id, \
    subject, \
    comment, \
    user_id, \
    user_trip, \
    user_country, \
    user_name, \
    birth_time, \
    change_time, \
    bump_time, \
    reply_count, \
    is_redacted, \
    is_saged \
    FROM ( \
        SELECT \
	id, \
	board, \
	thread_id, \
	subject, \
	comment, \
	user_id, \
	user_trip, \
	user_country, \
	user_name, \
	birth_time, \
	change_time, \
	bump_time, \
	reply_count, \
	is_redacted, \
	is_saged, \
	ROW_NUMBER() OVER (PARTITION BY thread_id ORDER BY id DESC) as rn \
	FROM Post \
	WHERE thread_id = ANY($1) \
    ) sub \
    WHERE rn <= 5 \
    ORDER BY id ASC \
    ";

    let replies = sqlx::query_as::<_, Post>(q)
	.bind(thread_ids)
	.fetch_all(&pool)
	.await?;

    let threads = Threads::from((threads, replies));

    let Threads { threads } = threads;

    Ok((threads, has_more, limit, last_bump_time))
}

pub fn country_code (
    ip: IpAddr
) -> Result<String, Box<dyn Error>> {
    let reader = Reader::open_readfile(std::env::var("GEOIP_URL")?)?;

    let result = reader.lookup(ip)?;

    if let Some(country_code) = result.decode_path(&[
	PathElement::Key("country"),
	PathElement::Key("iso_code"),
    ])? {
	return Ok(country_code)
    };

    eprintln!("Country code not found: {:?}", result);

    Ok( "ZZ".to_string() )
}

pub async fn post_with_id (
    id: i64,
    pool: PgPool,
) -> Result<Option<Post>, Box<dyn Error>> {
    let q = "\
    SELECT \
    id, \
    board, \
    thread_id, \
    subject, \
    comment, \
    user_id, \
    user_trip, \
    user_country, \
    user_name, \
    birth_time, \
    change_time, \
    bump_time, \
    reply_count, \
    is_redacted, \
    is_saged \
    FROM Post \
    WHERE id = $1 \
    ";

    Ok (
	sqlx::query_as::<_, Post>(q)
	    .bind(&id)
	    .fetch_optional(&pool)
	    .await?
    )
}

pub async fn ctx_up_sidebar (
    pool: PgPool,
    boards_cache: crate::BoardsCache,
    ctx: &mut tera::Context,
)  {
    let hobbies = boards_in_category(
	"Hobbies",
	pool.clone(),
	boards_cache.clone(),
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("hobbies", &hobbies);

    let interests = boards_in_category(
	"Interests",
	pool.clone(),
	boards_cache.clone(),
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("interests", &interests);

    let lifestyle = boards_in_category(
	"Lifestyle",
	pool.clone(),
	boards_cache.clone(),
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("lifestyle", &lifestyle);

    let local = boards_in_category(
	"Local",
	pool.clone(),
	boards_cache.clone(),
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("local", &local);

    let misc = boards_in_category(
	"Misc",
	pool.clone(),
	boards_cache,
    ).await.unwrap_or(
	vec![]
    );
    ctx.insert("misc", &misc);
}
