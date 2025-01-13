use actix_web::{get, HttpResponse, Responder};
use serde_json::{json, Value};

use crate::helper;

#[utoipa::path(responses((status = OK, body = String)))]
#[get("/akses_ssh")]
pub async fn akses_ssh() -> impl Responder {
    println!("akses_ssh");
    helper::ssh_access::initialize_ssh().await;
    println!("akses_ssh exit");
    HttpResponse::Ok().json(json!("Lihat Log"))
}