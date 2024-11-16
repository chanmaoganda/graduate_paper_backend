mod api;
mod model;
mod postgres;
mod services;
mod manager;

use actix_web::{web, App, HttpServer};
use manager::RegexManager;

fn get_addr_port<'a>() -> (&'a str, u16) {
    let address = env!("ADDRESS");
    let port = env!("PORT").parse().unwrap();
    (address, port)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::debug!("Starting server");

    let (address, port) = get_addr_port();
    let pool = postgres::build_pool().await.unwrap();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(RegexManager::new()))
            .configure(api::configure_apis)
    })
    .bind((address, port))?
    .run()
    .await
}
