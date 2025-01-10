mod tulis_excel;
use serde_json::json;
use tulis_excel::*;
mod permintaan_http;
use permintaan_http::*;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio;
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    fn_satu();
    fn_tulis_xlsx();
    println!("Berhasil menulis!");
    // fn_dua().await;
    // http_request::fn_akses_http().await;
    let a = fn_akses_http().await.unwrap_or(json!({"error":"something wrong"}));
    println!("Hasil Dua {:#?}", a);

    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8999))?
    .run()
    .await
}

