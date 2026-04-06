use serde::Deserialize;

#[derive(Deserialize)]
pub struct ThreadForm {
    pub subject: String,
    pub comment: String,
}

#[derive(Deserialize)]
pub struct ReplyForm {
    pub comment: String,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct ModerationForm {
    pub username: String,
    pub passphrase: String,
    pub redact: Option<String>,
    pub suspend: Option<String>,
    pub reason: String,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct TokenForm {
    pub server_pin: String,
    pub server_passphrase: String,
    pub token_passphrase: String,
}
