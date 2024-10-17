use axum::{routing::post, Router};
use sea_orm::DatabaseConnection;

use crate::handlers::customer_handler::post_customer;

pub fn customer_routes() -> Router<DatabaseConnection> {
    Router::new().route("/", post(post_customer))
}
