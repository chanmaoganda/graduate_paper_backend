mod api;
mod manager;
mod model;
mod postgres;
// mod services;

use std::sync::Arc;

use axum::{Extension, Router};
use manager::RegexManager;

fn get_addr_port() -> String {
    let address = env!("ADDRESS");
    let port = env!("PORT");
    format!("{}:{}", address, port)
}

// TODO: Add local timer for axum tracing_subscriber::fmt().with_timer()
#[tokio::main]
async fn main() {
    env_logger::init();

    let pool = postgres::build_pool().await.unwrap();

    let shared_pool = Arc::new(pool);
    let regex_manager = Arc::new(RegexManager::new());

    let app = Router::new()
        .nest("/api", api::registered_apis_router())
        .layer(Extension(shared_pool))
        .layer(Extension(regex_manager));

    let listener = tokio::net::TcpListener::bind(get_addr_port())
        .await
        .unwrap();

    log::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
