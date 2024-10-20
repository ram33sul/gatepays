use axum::{routing::post, Router};
use sea_orm::DatabaseConnection;

use crate::handlers::order_handler::post_order;

pub fn order_routes() -> Router<DatabaseConnection> {
    Router::new().route("/", post(post_order))
}
