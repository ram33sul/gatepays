use axum::{routing::post, Router};
use sea_orm::DatabaseConnection;

use crate::handlers::connector_handler::post_connector;

pub fn connector_routes() -> Router<DatabaseConnection> {
    Router::new().route("/", post(post_connector))
}
