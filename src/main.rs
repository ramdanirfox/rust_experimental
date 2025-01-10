use actix_cors::Cors;
use actix_web::{error, http::header, HttpResponse};
use rust_experimental::path;
// use serde_json::json;
mod permintaan_http;
// use permintaan_http::*;
// use tokio::net::TcpListener;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use dotenvy::dotenv;
// use tokio;
use actix_web::{get, web, App, HttpServer, Responder};

// #[get("/hello/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     format!("Hello {}!", name)
// }

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // println!("Hello, world!");
    // fn_satu();
    // fn_tulis_xlsx();
    // println!("Berhasil menulis!");
    // // fn_dua().await;
    // // http_request::fn_akses_http().await;
    // let a = fn_akses_http()
    //     .await
    //     .unwrap_or(json!({"error":"something wrong"}));
    // println!("Hasil Dua {:#?}", a);

    println!("Server Berjalan 8999");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT", "PATCH", "OPTIONS"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(7200);

        let json_cfg = web::JsonConfig::default()
            // limit request payload size
            .limit(104857600)
            .error_handler(|err: actix_web::error::JsonPayloadError, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
            });

        // App::new().service(greet)
        App::new()
            .wrap(cors)
            .configure(path::config)
            .app_data(json_cfg)
    })
    .bind(("0.0.0.0", 8999))?
    .run()
    .await
}
