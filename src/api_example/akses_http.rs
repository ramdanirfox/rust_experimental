use actix_web::{get, HttpResponse, Responder};
use serde_json::{json, Value};

use crate::helper;

#[utoipa::path(responses((status = OK, body = String)))]
#[get("/akses_http_net")]
pub async fn akses_http_net() -> impl Responder {
    let url = "https://raw.githubusercontent.com/ramdanirfox/rust_experimental/refs/heads/main/src/main.rs";
    // let v: Value = helper::permintaan_http::fn_akses_http(url.to_string()).await.unwrap_or(json!({"status": "Error"}));
    let v: Value = helper::permintaan_http::fn_akses_http(url.to_string()).await.expect("Rusak!");
    HttpResponse::Ok().json(v)
}

#[utoipa::path(responses((status = OK, body = String)))]
#[get("/akses_http_env")]
pub async fn akses_http_env() -> impl Responder {
    let url = std::env::var("HTTP_ENDPOINT_TEST").unwrap_or_default();
    let v: Value = helper::permintaan_http::fn_akses_http(url.to_string()).await.expect("reqwest Error");
    HttpResponse::Ok().json(v)
}   