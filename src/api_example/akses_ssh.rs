use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

use crate::helper;

#[utoipa::path(responses((status = OK, body = String)))]
#[get("/akses_ssh")]
pub async fn akses_ssh() -> impl Responder {
    println!("akses_ssh");
    let e = helper::ssh_access::initialize_ssh().await;
    match e {
        Ok(_) => println!("SSH Normal Exit"),
        Err(e) => println!("SSH Error Exit: {}", e),
    }
    println!("akses_ssh exit");
    HttpResponse::Ok().json(json!("Lihat Log"))
}