// GET: /stickers 
// GET: /sticker/{id}
// POST: /sticker (Add Sticker)
// POST: /sticker/{id} (Update Sticker)
// POST: /sticker/{id}/del (Delete Sticker)

use actix_web::{HttpResponse, get, post, web};
use bson::{doc, oid::ObjectId};
use futures::{TryStreamExt};
use mongodb::{Collection, Client};
use serde::Deserialize;
use serde_json::json;


use crate::{models::sticker::Sticker, errors::{LynixError}};

#[get("/stickers")]
async fn get_stickers(db: web::Data<Client>) -> Result<HttpResponse, LynixError> {
    let collection: Collection<Sticker> = db.database("lynix").collection("stickers");
    let mut cursor = collection.find(None, None).await?;

    let mut stickers: Vec<Sticker> = Vec::new();

    while let Ok(Some(mut sticker)) = cursor.try_next().await {
        sticker.base64 = None; // Set the base64 field to None to remove it from the struct definition
        stickers.push(sticker);
    }

    Ok(HttpResponse::Ok().json(stickers))
}

#[get("/sticker/{id}")]
async fn get_sticker_from_id(db: web::Data<Client>, path: web::Path<String>) -> Result<HttpResponse, LynixError> {
    let collection: Collection<Sticker> = db.database("lynix").collection("stickers");
    let id = path.into_inner();

    //let obj_id = ObjectId::parse_str(&id).map_err(|_| LynixError::InternalError)?;
    let filter = doc! {"sid": id};
    
    println!("Sticker Filter: {:?}", &filter);

    let content_type = "image/png";

    // Get the sticker from the database
    if let Ok(Some(sticker)) = collection.find_one(filter, None).await {
        let base64_data = sticker.base64.unwrap_or_default();
        let image_data = base64::decode(base64_data);
        
        // Sticker found, return it
        Ok(HttpResponse::Ok()
        .content_type(content_type)
        .body(image_data?))
    } else {
        // Sticker not found, return 404
        Err(LynixError::NotFound)
    }
}

/* Add Sticker */
#[post("/sticker")]
async fn add_sticker(db: web::Data<Client>) -> Result<HttpResponse, LynixError> {
    Ok(HttpResponse::Ok().json(json!({"msg": "Not Implemented!"})))
}

/* Update Sticker */
#[derive(Debug, Deserialize)]
struct StickerUpdate {
    base64: String,
}

#[post("/sticker/{id}")]
async fn update_sticker(db: web::Data<Client>, path: web::Path<String>, sticker_update: web::Json<StickerUpdate>) -> Result<HttpResponse, LynixError> {
    let collection: Collection<Sticker> = db.database("lynix").collection("stickers");
    let id = path.into_inner();

    let obj_id = ObjectId::parse_str(&id).map_err(|_| LynixError::InternalError)?;
    let filter = doc! {"_id": obj_id};
    let update = doc! {"$set": {"base64": &sticker_update.base64}};

    let result = collection.update_one(filter, update, None).await.map_err(|_| LynixError::InternalError)?;

    if result.modified_count > 0 {
        Ok(HttpResponse::Ok().json(json!({"msg": "Sticker Updated!"})))
    } else {
        Err(LynixError::BadClientData)
    }
}

/* Delete Sticker */
#[post("/sticker/{id}/del")]
async fn delete_sticker(db: web::Data<Client>, path: web::Path<String>) -> Result<HttpResponse, LynixError> {
    let collection: Collection<Sticker> = db.database("lynix").collection("stickers");
    let id = path.into_inner();

    //let obj_id = ObjectId::parse_str(&id).map_err(|_| LynixError::InternalError)?;
    let filter = doc! {"sid": id};

    Ok(HttpResponse::Ok().json(json!({"msg": "Not Implemented!"})))
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(get_stickers)
            .service(get_sticker_from_id)
            .service(add_sticker)
            .service(update_sticker)
            .service(delete_sticker)
    );
}