use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::db::DataBase;
use crate::db::{CreateTaskRequest};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "ok"
    }))
}

pub async fn add_task(db: web::Data<DataBase>, req: web::Json<CreateTaskRequest>) -> HttpResponse {
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

pub async fn list_tasks(db: web::Data<DataBase>) -> HttpResponse {
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

pub async fn get_task(db: web::Data<DataBase>, path: web::Path<i64>) -> HttpResponse {
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

pub async fn delete_task(db: web::Data<DataBase>, path: web::Path<i64>) -> HttpResponse {
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

pub async fn complete_task(db: web::Data<DataBase>, path: web::Path<i64>) -> HttpResponse {
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