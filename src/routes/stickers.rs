// GET: /stickers 
// GET: /sticker/{id}
// POST: /sticker (Add Sticker)
// POST: /sticker/{id} (Update Sticker)
// POST: /sticker/{id}/del (Delete Sticker)

use std::sync::Arc;

use actix_web::{HttpResponse, get, post, web};
use bson::{doc, oid::ObjectId};
use mongodb::{Collection, Client}; // Will be removed soon
use scylla::{Session, IntoTypedRows, FromRow};
use serde::Deserialize;
use serde_json::json;


use crate::{models::sticker::Sticker, errors::{LynixError}};

#[get("/stickers")]
async fn get_stickers(db: web::Data<Arc<Session>>) -> Result<HttpResponse, LynixError> {
    let query = "SELECT * FROM lynixca.stickers";

    /*let result_set = match db.query(query, &[]).await {
        Ok(result_set) => result_set,
        Err(err) => {
            eprintln!("Failed to execute query: {:?}", err);
            return Err(LynixError::BadData(err.to_string()));
        }
    };*/

    let mut stickers: Vec<Sticker> = Vec::new();

    if let Some(rows) = db.query(query, &[]).await.unwrap().rows {
        for row in rows { // Figure out how to make this into a Sticker object and push it to the vector to return JSON
            let sticker = Sticker::from_row(row).unwrap();
            println!("Sticker Row, {}", sticker.sid);
        }
    }

    Ok(HttpResponse::Ok().json(json!(stickers)))
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
        let image_data = base64::decode(base64_data); // This needs to be resolved soon.
        
        // Sticker found, return it
        Ok(HttpResponse::Ok()
        .content_type(content_type)
        .body(image_data?))
    } else {
        // Sticker not found, return 404
        Err(LynixError::NotFound)
    }
}

/* Add Sticker
    ⚠️ This path is disabled due to security concerns with authentication api. ⚠️
*/
#[post("/sticker")]
async fn add_sticker() -> Result<HttpResponse, LynixError> {
    Err(LynixError::Unauthorized)
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

/* Delete Sticker
    ⚠️ This path is disabled due to security concerns with authentication api. ⚠️
*/
#[post("/sticker/{id}/del")]
async fn delete_sticker() -> Result<HttpResponse, LynixError> {
    Err(LynixError::Unauthorized)
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_stickers)
        .service(get_sticker_from_id)
        .service(add_sticker)
        .service(update_sticker)
        .service(delete_sticker);
}