use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::db::DataBase;
use crate::db::{CreateTaskRequest};

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
        "status": "ok"
    }))
}

async fn add_task(db: web::Data<DataBase>, req: web::Json<CreateTaskRequest>) -> HttpResponse {
    match db.insert_task(&req) {
        Ok(task_id) => {
            HttpResponse::Ok().json(json!({
                "message": "Task added successfully",
                "id": task_id,
                "text": req.text
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to add task",
                "details": e.to_string()
            }))
        }
    }
}