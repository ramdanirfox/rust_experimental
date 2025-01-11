use actix_cors::Cors;
use actix_web::{error, http::header, middleware::Logger, HttpResponse};
use rust_experimental::{path, shared};
mod permintaan_http;
use dotenvy::dotenv;
use actix_web::{web, App, HttpServer};
use utoipa_actix_web::AppExt;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

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
        let (app, api) = App::new()
            .into_utoipa_app()
            .configure(path::config)
            // .service(scope::scope("/"))
            // .split_for_parts()
            // .openapi(ApiDoc::openapi())
            .map(|app| {
                app.wrap(Logger::default())
                .wrap(cors)
                .app_data(json_cfg)
            })
            .split_for_parts();
        // println!("{}", api.to_pretty_json().unwrap());
        let json_value: serde_json::Value = serde_json::from_str(&api.to_json().unwrap().to_string()).unwrap();
        shared::set_data(json_value);
        app
        
    })
    .bind(("0.0.0.0", 8999))?
    .run()
    .await
}
