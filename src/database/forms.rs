use serde::Deserialize;

// #[derive(Deserialize)]
// pub struct PostForm {
//     pub comment: String,
// }

#[derive(Deserialize)]
pub struct ThreadForm {
    pub subject: String,
    pub comment: String,
}
