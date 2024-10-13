use axum::{routing::post, Router};
use sea_orm::DatabaseConnection;

use crate::handlers::paypal_handlers::post_order;

pub fn paypal_routes() -> Router<DatabaseConnection> {
    Router::new().route("/order", post(post_order))
}
