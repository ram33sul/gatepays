use axum::{
    routing::{post, put},
    Router,
};
use sea_orm::DatabaseConnection;

use crate::handlers::merchant_handler::{post_merchant, toggle_merchant};

pub fn merchant_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", post(post_merchant))
        .route("/toggle", put(toggle_merchant))
}
