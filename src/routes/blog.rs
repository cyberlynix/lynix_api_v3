use actix_web::{web, get, HttpResponse, post};
use bson::{oid::ObjectId, doc};
use futures::TryStreamExt;
use mongodb::{Collection, Client};

use crate::{models::article::Article, errors::LynixError};

/* Get all articles */
#[get("/articles")]
async fn get_articles(db: web::Data<Client>) -> Result<HttpResponse, LynixError> {
    let collection: Collection<Article> = db.database("lynix").collection("events");
    let mut cursor = collection.find(None, None).await?;

    let mut events: Vec<Article> = Vec::new();

    while let Ok(Some(event)) = cursor.try_next().await {
        events.push(event);
    }

    Ok(HttpResponse::Ok().json(events))
}

/* Get single article */
#[get("/article/{id}")]
async fn get_article_by_id(db: web::Data<Client>, path: web::Path<String>) -> Result<HttpResponse, LynixError> {
    let collection: Collection<Article> = db.database("lynix").collection("events");
    let id = path.into_inner();

    let obj_id = ObjectId::parse_str(&id).map_err(|_| LynixError::InternalError)?;
    let filter = doc!{"_id": obj_id};

    if let Ok(Some(event)) = collection.find_one(filter, None).await {
        Ok(HttpResponse::Ok().json(event))
    } else {
        Err(LynixError::NotFound)
    }
}

/* Add Article 
    ⚠️ This path is disabled due to security concerns with authentication api. ⚠️
*/
#[post("/article")]
async fn add_article() -> Result<HttpResponse, LynixError> {
    Err(LynixError::Unauthorized)
}

/* Update Article 
    ⚠️ This path is disabled due to security concerns with authentication api. ⚠️
*/
#[post("/article/{id}")]
async fn update_article() -> Result<HttpResponse, LynixError> {
    Err(LynixError::Unauthorized)
}

/* Delete Article 
    ⚠️ This path is disabled due to security concerns with authentication api. ⚠️
*/
#[post("/article/{id}/del")]
async fn delete_article() -> Result<HttpResponse, LynixError> {
    Err(LynixError::Unauthorized)
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_articles);
    cfg.service(get_article_by_id);
    cfg.service(add_article);
    cfg.service(update_article);
    cfg.service(delete_article);
}