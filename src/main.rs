use actix_web::{App, HttpServer};
use actix_cors::Cors;

mod routes;

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
            .configure(routes::config)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
