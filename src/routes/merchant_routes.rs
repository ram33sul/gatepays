use axum::{routing::post, Router};
use sea_orm::DatabaseConnection;

use crate::handlers::merchant_handler::post_merchant;

pub fn merchant_routes() -> Router<DatabaseConnection> {
    Router::new().route("/", post(post_merchant))
}
