use scylla::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Sticker {
    pub sid: String,
    pub base64: Option<String>,
}