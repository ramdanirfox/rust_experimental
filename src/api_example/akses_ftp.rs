use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

use crate::helper;

#[utoipa::path(responses((status = OK, body = String)))]
#[get("/akses_ftp")]
pub async fn akses_ftp() -> impl Responder {
    println!("akses_ftp");
    helper::ftp_access::init_ftp();
    println!("akses_ftp exit");
    HttpResponse::Ok().json(json!("Lihat Log untuk FTP. note that FTPS is not supported yet until suppaftp provide a way to use rustls with custom provider"))
}