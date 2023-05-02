use actix_web::{HttpResponse, get, post, web};
use bson::doc;
use mongodb::{Client, Collection};
use serde_json::json;
use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Pbkdf2
};
use crate::models::user::{UserRegisterSchema, User, UserLoginSchema};

use crate::errors::LynixError;

/* Login */
#[post("/auth/signin")]
async fn signin(db: web::Data<Client>, data: web::Json<UserLoginSchema>) -> Result<HttpResponse, LynixError> {
    let collection: Collection<User> = db.database("lynix").collection("users");
    let filter = doc! {"email": &data.email};

    println!("[AUTH] Email '{}' is currently signing in.", &data.email);

    let user = match collection.find_one(filter, None).await {
        Ok(Some(row)) => row,
        Ok(None) => return Err(LynixError::NotFound), //User Not Found
        Err(err) => return Err(LynixError::BadData(err.to_string())),
    };

    // Verify password
    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    if let Err(_err) = Pbkdf2.verify_password(data.password.as_bytes(), &parsed_hash) {
        return Err(LynixError::BadData(("Invalid Credentials".to_string())));
    }

    // TODO: Create Session

    Ok(HttpResponse::Ok().body(format!("Welcome, {}", user.username)))
}

/* Register */
#[post("/auth/signup")]
async fn signup(db: web::Data<Client>, data: web::Json<UserRegisterSchema>) -> Result<HttpResponse, LynixError> {

    // Check if user exists
    let collection: Collection<User> = db.database("lynix").collection("users");
    let filter = doc! {"email": &data.email};

    let user = collection.find_one(filter, None).await?;

    if let Some(_) = user {
        return Err(LynixError::BadData("User already exists".to_string()));
    }

    println!("[INFO] User does not exist, creating...");

    /* Hash Password */
    let salt = SaltString::generate(&mut OsRng);
    // Hash password to PHC string ($pbkdf2-sha256$...)
    let hashed_password = Pbkdf2.hash_password(data.password.to_owned().as_bytes(), &salt).unwrap().to_string(); // Franko will scream!

    /* Register */
    // Convert UserRegisterSchema JSON to User (mismatched types expected struct `User`, found struct `UserRegisterSchema)
    let user = User {
        id: None,
        email: data.email.to_owned().to_lowercase(),
        username: data.username.to_owned(),
        password: hashed_password,
        avatar_url: None,
        is_furry: false,
        is_admin: false,
        is_suspended: false,

        otp_enabled: Some(false),
        otp_verified: Some(false),
        otp_base32: None,
        otp_auth_url: None,
    };

    collection.insert_one(&user, None).await?;

    Ok(HttpResponse::Ok().json(json!({ "status": "ok" })))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(signup);
    cfg.service(signin);
}