use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub location: String,
    pub image_url: String,

    pub start_time: i64, // Unix 
    pub end_time: i64, // Unix

    #[serde(default = "default_false")]
    pub is_public: bool,
    #[serde(default = "default_false")]
    pub is_featured: bool,
    #[serde(default = "default_false")]
    pub is_furry: bool, // Super Secret Area Access Required

    #[serde(default)]
    pub status: i32,
}

fn default_false() -> bool {
    false
}