mod api;
mod model;
mod postgres;
mod services;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = postgres::build_pool().await.unwrap();
    env_logger::init();
    log::debug!("Starting server");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::configure_apis)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
