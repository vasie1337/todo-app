use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let port: u16 = 8080;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )
            .route("/health", web::get().to(health_check))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
