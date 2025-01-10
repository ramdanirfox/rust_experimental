use actix_web::{web, HttpResponse, Responder};

use crate::api_example;

pub async fn tes() -> impl Responder {
    HttpResponse::Ok().json("ok")
}

pub async fn daftar_api() -> impl Responder {
    HttpResponse::Ok().json("API : baca_tulis_excel,")
}

pub fn config(cfg: &mut web::ServiceConfig) {

    cfg
        .route("/", web::get().to(daftar_api))
        .route("/test", web::get().to(tes))
        .route("/baca_tulis_excel", web::get().to(api_example::baca_tulis_excel::baca_tulis_excel))
        ;
}