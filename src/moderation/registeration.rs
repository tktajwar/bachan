use argon2::{
    Argon2,
    password_hash::{
        rand_core::OsRng,
	PasswordHasher,
	SaltString,
    },
};
use axum::extract::State;
use sqlx::PgPool;
use std::error::Error;
use std::io::Write;
use uuid::Uuid;

pub async fn register_mod(
    State(pool): State<PgPool>,
    username: &str,
    passphrase: &[u8],
) -> Result<i32, Box<dyn Error>> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let Ok(pass_hash) = argon2.hash_password(passphrase, &salt) else {
	return Err(Box::<dyn Error>::from("Argon couldn't hash passphrase"))
    };

    let q = "\
    INSERT INTO mod \
    (username, hash) VALUES \
    ($1, $2)\
    RETURNING id \
    ";

    let id: (i32,) = sqlx::query_as(q)
	.bind(username)
	.bind(pass_hash.to_string())
	.fetch_one(&pool)
	.await?;

    Ok(id.0)
}

#[allow(unused)]
pub async fn register_mod_from_cli(
    state_pool: State<PgPool>,
) -> Result<i32, Box<dyn Error>> {
    let mut username = String::new();

    print!("Username: ");

    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    let username = username.trim();

    let mut passphrase = String::new();

    print!("Passphrase: ");

    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut passphrase)
        .expect("Failed to read line");

    let passphrase = passphrase.trim();

    let id = register_mod(state_pool, username, passphrase.as_bytes()).await?;

    Ok(id)
}

pub async fn create_mod_token (
    State(pool): State<PgPool>,
    passphrase: &[u8],
) -> Result<Uuid, Box<dyn Error>> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let Ok(pass_hash) = argon2.hash_password(passphrase, &salt) else {
	return Err(Box::<dyn Error>::from("Argon couldn't hash passphrase"))
    };

    let q = "\
    INSERT INTO ModToken \
    (hash) VALUES \
    ($1)\
    RETURNING id \
    ";

    let id: (Uuid,) = sqlx::query_as(q)
	.bind(pass_hash.to_string())
	.fetch_one(&pool)
	.await?;

    Ok(id.0)
}
