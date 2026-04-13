use serde::Deserialize;

#[derive(Deserialize)]
pub struct ThreadForm {
    pub subject: String,
    pub comment: String,
}

#[derive(Deserialize)]
pub struct PopupThreadForm {
    pub subject: String,
    pub comment: String,
    pub board: String,
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

#[derive(Deserialize)]
#[derive(Debug)]
pub struct RegisterationForm {
    pub token_passphrase: String,
    pub username: String,
    pub passphrase: String,
}

#[derive(Deserialize)]
pub struct PaginationWithID {
    pub limit: Option<i32>,
    pub before_id: Option<String>,
}

#[derive(Deserialize)]
pub struct PaginationWithMTime {
    pub limit: Option<i32>,
    pub before_mtime: Option<i64>,
}

#[derive(Deserialize)]
pub struct SubmissionForm {
    pub action: String,
}
