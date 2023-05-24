// controllers.rs

use actix_web::{ HttpResponse };
use serde_json::json;

pub async fn home() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "message": "success",
        "data":[]
    }))
}