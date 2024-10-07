mod config;
mod db;
mod handlers;
mod models;
mod routes;

use axum::Server;
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();
    let db = db::init_db(&config.database_url).await;

    let app = routes::create_routes(db);

    let addr = SocketAddr::from((config.server_host, config.server_port));
    tracing::info!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
