use actix_web::{HttpResponse, get, post, web};
use serde_json::json;

use crate::errors::LynixError;
/* Login */
#[post("/signin")]
async fn signin() -> Result<HttpResponse, LynixError> {
    Ok(HttpResponse::Ok().json(json!({ "status": "ok" })))
}

/* Register */
#[post("/signup")]
async fn signup() -> Result<HttpResponse, LynixError> {
    Ok(HttpResponse::Ok().json(json!({ "status": "ok" })))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/auth") // Force Auth Path
            .service(signup)
            .service(signin)
    );
}