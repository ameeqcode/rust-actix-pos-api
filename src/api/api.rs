use actix_web::{web, post, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{models::pos_items::POSItems, repository::database::Database};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct POSOuterArray {
  pub outer_array: Option<Vec<POSItems>>,
}


#[post("/pos-items")]
pub async fn import_pos_items_from_json(web::Json(data): web::Json<POSOuterArray>, db: web::Data<Database>) -> HttpResponse {
    if let Some(inner_objects) = data.outer_array {
            let _ = db.import_pos_items(inner_objects);
    }
    HttpResponse::Ok().json("POS items import successful.")
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(import_pos_items_from_json)
    );
}
