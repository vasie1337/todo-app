use actix_web::{web, HttpResponse};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::db::DataBase;
use crate::db::TaskEntry;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/health").route(web::get().to(health_check))
    )
    .service(
        web::resource("/tasks").route(web::post().to(add_task))
    );
}
 
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }))
}

async fn add_task(db: web::Data<DataBase>, req: web::Json<TaskEntry>) -> HttpResponse {
    HttpResponse::Ok().json(json!({"message": "Task added successfully"}))
}