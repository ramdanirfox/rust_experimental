use actix_web::{get, HttpResponse, Responder};

use crate::{api_example, shared};

// #[derive(OpenApi)]
// #[openapi(
//     info(description = "My Api description"),
// )]
// struct ApiDoc;

#[utoipa::path(responses((status = OK, body = String)))]
#[get("/tes")]
pub async fn tes() -> impl Responder {
    HttpResponse::Ok().json("API is ok")
}
#[utoipa::path(responses((status = OK, body = String)))]
#[get("/")]
pub async fn path_root() -> impl Responder {
    // HttpResponse::Ok().json(ApiDoc::openapi().to_pretty_json().unwrap())
    HttpResponse::Ok().json(shared::get_data().lock().unwrap().count.clone())
}


// pub fn config(cfg: &mut web::ServiceConfig) {
pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {

    cfg
        .service(path_root)
        .service(tes)
        .service(api_example::baca_tulis_excel::baca_tulis_excel)
        ;
}