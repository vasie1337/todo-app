use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::db::DataBase;
use crate::db::{CreateTaskRequest};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/health").route(web::get().to(health_check))
    )
    .service(
        web::resource("/tasks")
            .route(web::post().to(add_task))
            .route(web::get().to(list_tasks))
    )
    .service(
        web::resource("/tasks/{id}")
            .route(web::get().to(get_task))
            .route(web::delete().to(delete_task))
    )
    .service(
        web::resource("/tasks/{id}/complete")
            .route(web::patch().to(complete_task))
    );
}
 
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "ok"
    }))
}

async fn add_task(db: web::Data<DataBase>, req: web::Json<CreateTaskRequest>) -> HttpResponse {
    match db.insert(&req) {
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

async fn list_tasks(db: web::Data<DataBase>) -> HttpResponse {
    match db.get_all() {
        Ok(tasks) => {
            HttpResponse::Ok().json(json!({
                "tasks": tasks,
                "count": tasks.len()
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch tasks",
                "details": e.to_string()
            }))
        }
    }
}

async fn get_task(db: web::Data<DataBase>, path: web::Path<i64>) -> HttpResponse {
    let task_id = path.into_inner();
    
    match db.get_by_id(task_id) {
        Ok(Some(task)) => {
            HttpResponse::Ok().json(task)
        }
        Ok(None) => {
            HttpResponse::NotFound().json(json!({
                "error": "Task not found",
                "id": task_id
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch task",
                "details": e.to_string()
            }))
        }
    }
}

async fn delete_task(db: web::Data<DataBase>, path: web::Path<i64>) -> HttpResponse {
    let task_id = path.into_inner();
    
    match db.delete_task(task_id) {
        Ok(true) => {
            HttpResponse::Ok().json(json!({
                "message": "Task deleted successfully",
                "id": task_id
            }))
        }
        Ok(false) => {
            HttpResponse::NotFound().json(json!({
                "error": "Task not found",
                "id": task_id
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to delete task",
                "details": e.to_string()
            }))
        }
    }
}

async fn complete_task(db: web::Data<DataBase>, path: web::Path<i64>) -> HttpResponse {
    let task_id = path.into_inner();
    
    match db.complete_task(task_id) {
        Ok(true) => {
            HttpResponse::Ok().json(json!({
                "message": "Task completed successfully",
                "id": task_id
            }))
        }
        Ok(false) => {
            HttpResponse::NotFound().json(json!({
                "error": "Task not found",
                "id": task_id
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to complete task",
                "details": e.to_string()
            }))
        }
    }
}