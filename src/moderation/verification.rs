use argon2::{
    Argon2,
    password_hash::{
        PasswordHash,
	PasswordVerifier,
    },
};
use axum::extract::State;
use sqlx::PgPool;
use std::error::Error;
use std::io::Write;

pub async fn verify_mod(
    State(pool): State<PgPool>,
    username: &str,
    passphrase: &str,
) -> Result<bool, Box<dyn Error>> {
    let row: Option<(String,)> = sqlx::query_as(
        r#"
        SELECT hash 
        FROM mod 
        WHERE username = $1
        "#
    )
    .bind(username)
    .fetch_optional(&pool)
    .await?;

    let (stored_hash,) = match row {
        Some((hash,)) => (hash,),
        None => return Ok(false),
    };

    let argon2 = Argon2::default();
    let Ok(pass_hash) = PasswordHash::new(&stored_hash) else {
	return Err(Box::<dyn Error>::from("Argon couldn't hash passphrase"))
    };

    Ok(argon2
       .verify_password(passphrase.as_bytes(), &pass_hash)
       .is_ok()
    )
}

#[allow(unused)]
pub async fn verify_mod_from_cli(
    state_pool: State<PgPool>,
) -> Result<bool, Box<dyn Error>> {
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

    verify_mod(state_pool, username, passphrase).await
}
