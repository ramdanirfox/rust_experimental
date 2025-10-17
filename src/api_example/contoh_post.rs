use actix_web::{post, HttpResponse, Responder};
use serde_json::json;

#[utoipa::path(responses((status = OK, body = String)))]
#[post("/contoh_post")]
pub async fn contoh_post() -> impl Responder {
    println!("contoh_post");

    let is_mode_error = std::env::var("POST_ERROR").unwrap_or("no".to_string());

    if (is_mode_error == "no") {
        HttpResponse::Ok().json(json!({
            "code": "success"
        }))
    } else {
        HttpResponse::InternalServerError().json(json!({
            "code": "50001"
        }))
    }
}
