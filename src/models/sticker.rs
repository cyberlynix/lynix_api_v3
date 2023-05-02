use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Sticker {
    pub id: String,
    pub sid: String,
    pub base64: Option<String>,
}