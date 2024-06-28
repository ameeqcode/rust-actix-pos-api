use actix_web::{web, post, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{models::pos_items::POSItems, repository::database::Database};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct POSOuterArray {
  pub outer_array: Option<Vec<POSItems>>,
}


#[post("/pos-items")]
pub async fn pos_items_import(web::Json(data): web::Json<POSOuterArray>, db: web::Data<Database>) -> HttpResponse {
    if let Some(inner_objects) = data.outer_array {
            let _ = db.pos_items_imports(inner_objects);
    }
    HttpResponse::Ok().json("POS items import successful.")
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(pos_items_import)
    );
}
