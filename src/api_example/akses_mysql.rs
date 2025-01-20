use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

// WARNING : this is not part of rust experimental

use crate::helper;

#[utoipa::path(responses((status = OK, body = String)))]
#[get("/akses_mysql")]
pub async fn akses_mysql() -> impl Responder {
    println!("akses_mysql");
    let e = helper::mysql_access::connect_mysql().await;
    match e {
        Ok(_) => println!("MySQL Normal Exit"),
        Err(e) => println!("MySQL Error Exit: {}", e),
    }
    println!("akses_mysql exit");
    HttpResponse::Ok().json(json!("Lihat Log"))
}