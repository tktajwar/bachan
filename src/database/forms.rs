use serde::Deserialize;

#[derive(Deserialize)]
pub struct ThreadForm {
    pub subject: String,
    pub comment: String,
}
