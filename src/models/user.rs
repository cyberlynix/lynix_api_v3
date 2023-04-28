use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub is_furry: bool, // This is required for Super Secret Area
    pub is_admin: bool,
    pub is_suspended: bool,

    /* 2FA */
    pub otp_enabled: Option<bool>,
    pub otp_verified: Option<bool>,
    pub otp_base32: Option<String>,
    pub otp_auth_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserRegisterSchema {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserLoginSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateOTPSchema {
    pub email: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyOTPSchema {
    pub user_id: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct DisableOTPSchema {
    pub user_id: String,
}