use actix_web::{web};

use crate::handlers::*;

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
