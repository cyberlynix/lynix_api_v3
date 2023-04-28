use actix_web::{HttpResponse, get, post, web};
use bson::doc;
use mongodb::{Client, Collection};
use serde_json::json;
use crate::models::user::{UserRegisterSchema, User, UserLoginSchema};

use crate::errors::LynixError;
/* Login */
#[post("/auth/login")]
async fn signin(db: web::Data<Client>, data: web::Json<UserLoginSchema>) -> Result<HttpResponse, LynixError> {
    let collection: Collection<User> = db.database("lynix").collection("users");
    let filter = doc! {"email": &data.email};

    // Check if user exists
    let user = collection.find_one(filter, None).await?;

    if let Some(_) = user {
       // Compare password
       if user.unwrap().password == data.password {
              // Passwords match, return token
              return Ok(HttpResponse::Ok().json(json!({ "status": "Login Success!" })));
         } else {
              // Passwords don't match, return error
              return Ok(HttpResponse::Ok().json(json!({ "status": "Invalid Credentials." })));
       }
    }

    Ok(HttpResponse::Ok().body("Hey"))
}

/* Register */
#[post("/auth/register")]
async fn register(db: web::Data<Client>, data: web::Json<UserRegisterSchema>) -> Result<HttpResponse, LynixError> {

    // Check if user exists
    let collection: Collection<User> = db.database("lynix").collection("users");
    let filter = doc! {"email": &data.email};

    let user = collection.find_one(filter, None).await?;

    if let Some(_) = user {
        return Err(LynixError::BadData("User already exists".to_string()));
    }

    /* Register */
    // Convert UserRegisterSchema JSON to User (mismatched types expected struct `User`, found struct `UserRegisterSchema)
    let user = User {
        id: None,
        email: data.email.to_owned().to_lowercase(),
        username: data.username.to_owned(),
        password: data.password.to_owned(),
        avatar_url: None,
        is_furry: false,
        is_admin: false,
        is_suspended: false,

        otp_enabled: Some(false),
        otp_verified: Some(false),
        otp_base32: None,
        otp_auth_url: None,
    };

    let user = collection.insert_one(&user, None).await?;

    println!("Registering user: {:?}", &data);
    Ok(HttpResponse::Ok().json(json!({ "status": "ok" })))
}

/* Generate OTP */
#[post("/auth/otp/generate")]
pub fn generate() -> Result<HttpResponse, LynixError> {
    Ok(HttpResponse::Ok().json(json!({ "status": "ok" })))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v2") // Force Auth Path
            .service(register)
            .service(signin)
    );
}