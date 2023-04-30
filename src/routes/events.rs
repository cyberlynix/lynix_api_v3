use actix_web::{web, get, HttpResponse, post};
use bson::{oid::ObjectId, doc};
use futures::TryStreamExt;
use mongodb::{Collection, Client};

use crate::{models::event::Event, errors::LynixError};

/* Get all events */
#[get("/events")]
async fn get_events(db: web::Data<Client>) -> Result<HttpResponse, LynixError> {
    let collection: Collection<Event> = db.database("lynix").collection("events");
    let mut cursor = collection.find(None, None).await?;

    let mut events: Vec<Event> = Vec::new();

    while let Ok(Some(event)) = cursor.try_next().await {
        events.push(event);
    }

    Ok(HttpResponse::Ok().json(events))
}

/* Get single event */
#[get("/event/{id}")]
async fn get_event_by_id(db: web::Data<Client>, path: web::Path<String>) -> Result<HttpResponse, LynixError> {
    let collection: Collection<Event> = db.database("lynix").collection("events");
    let id = path.into_inner();

    let obj_id = ObjectId::parse_str(&id).map_err(|_| LynixError::InternalError)?;
    let filter = doc!{"_id": obj_id};

    if let Ok(Some(event)) = collection.find_one(filter, None).await {
        Ok(HttpResponse::Ok().json(event))
    } else {
        Err(LynixError::NotFound)
    }
}

/* Add Event 
    ⚠️ This path is disabled due to security concerns with authentication api. ⚠️
*/
#[post("/event")]
async fn add_event() -> Result<HttpResponse, LynixError> {
    Err(LynixError::Unauthorized)
}

/* Update Event 
    ⚠️ This path is disabled due to security concerns with authentication api. ⚠️
*/
#[post("/event/{id}")]
async fn update_event() -> Result<HttpResponse, LynixError> {
    Err(LynixError::Unauthorized)
}

/* Delete Event 
    ⚠️ This path is disabled due to security concerns with authentication api. ⚠️
*/
#[post("/event/{id}/del")]
async fn delete_event() -> Result<HttpResponse, LynixError> {
    Err(LynixError::Unauthorized)
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_events);
    cfg.service(get_event_by_id);
    cfg.service(add_event);
    cfg.service(update_event);
    cfg.service(delete_event);
}