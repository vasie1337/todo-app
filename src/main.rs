use actix_web::{App, HttpServer};
use actix_cors::Cors;

mod routes;
mod db;

const APP_PORT: u16 = 8080;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = db::DataBase::open("db.sql").expect("Failed to open the database");
    db.migrate().expect("Failed to migrate the database");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )
            .app_data(actix_web::web::Data::new(db.clone()))
            .configure(routes::config)
    })
    .bind(("0.0.0.0", APP_PORT))?
    .run()
    .await
}
