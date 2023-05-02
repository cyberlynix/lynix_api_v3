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
use scylla::Session;
use serde_json::json;
use std::{arch::x86_64, sync::Arc};

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
async fn check_db_conn(db: web::Data<Arc<Session>>) -> Result<HttpResponse, LynixError> {
    // Use the ScyllaDB session to perform a simple query or connection check
    // Here's an example of checking the connection by executing a SELECT query
    let query = "SELECT * FROM lynixca.stickers LIMIT 1";
    match db.query(query, &[]).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Connection to ScyllaDB is successful")),
        Err(err) => {
            eprintln!("Failed to check DB connection: {:?}", err);
            Err(LynixError::BadData(("Failed to check DB connection").to_string()))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up env
    dotenv().ok();

    // Print Lynix ASCII Art
    println!("---------------------------------------");
    println!("Lynix API");
    println!("---------------------------------------");

    // Set up logger
    env_logger::init();

    // Check AVX CPU Support
    if !is_x86_feature_detected!("avx") {
        println!("WARNING: AVX is not supported and is recommended for use with the LynixAPI!");
    }

    // Set up ScyllaDB client
    let session = db::init().await;
    let session = Arc::new(session);

    HttpServer::new(move || { 
        App::new()
        .app_data(Data::new(session.clone()))
        /* v1 paths */
        .service(
            web::scope("/v1")
                .configure(stickers::configure_routes)
                .configure(events::configure_routes)
                .configure(blog::configure_routes)
                .configure(auth::configure_routes)
        )
        .service(check_db_conn)
        .default_service(web::route().to(handle_404))
    }).bind(("0.0.0.0", 28300))?.run().await


}

async fn handle_404() -> Result<HttpResponse, errors::LynixError> {
    Err(LynixError::NotFound)
}