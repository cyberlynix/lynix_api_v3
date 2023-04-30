use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub sid: String, // URL ID
    pub image_url: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub tags: Vec<String>,
    pub author: Option<ObjectId>,

    // Timestamps
    pub created_at: i64, // Unix
    pub modified_at: i64, // Unix

    #[serde(default = "default_false")]
    pub is_public: bool,
}

fn default_false() -> bool {
    false
}