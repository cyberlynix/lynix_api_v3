mod middleware;
mod routes;
mod errors;
mod models;
mod db;

use actix_web::{get, web::{self, Data}, App, HttpServer, HttpResponse, Responder};
use errors::LynixError;
use mongodb::Client;
use routes::{stickers, auth, events, blog};
use dotenvy::dotenv;
use serde_json::json;

/* Example from Actix */
#[get("/")]
async fn index() -> impl Responder {
    // Multiple Line String
    "Hello World!
    This is a test!
    This is a test!
    This is a test!
    ".to_string()
}

/* Example from Actix */
#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[get("/check_db_conn")]
async fn check(db: web::Data<Client>) -> Result<HttpResponse, LynixError> {
    // Check if MongoDB is online
    match db.list_database_names(None, None).await {
        Ok(_) => {
            let response = json!({ "db_online": true, "available_databases": db.list_database_names(None, None).await? });
            Ok(HttpResponse::Ok().json(response))
        },
        Err(_) => {
            let response = json!({ "db_online": false });
            Ok(HttpResponse::Ok().json(response))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up env
    dotenv().ok();

    // Set up logger
    env_logger::init();

    // Set up MongoDB client
    let client = db::init().await;

    HttpServer::new(move || { 
        App::new()
        .app_data(Data::new(client.clone()))
        /* v1 paths */
        .service(
            web::scope("/v1")
                .configure(stickers::configure_routes)
                .configure(auth::configure_routes)
                .configure(events::configure_routes)
                .configure(blog::configure_routes)
        )
        .default_service(web::route().to(handle_404))
    }).bind(("0.0.0.0", 28300))?.run().await


}

async fn handle_404() -> Result<HttpResponse, errors::LynixError> {
    Err(LynixError::NotFound)
}