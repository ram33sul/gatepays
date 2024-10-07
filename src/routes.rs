use crate::handlers::user_handler::create_user;
use axum::{routing::post, Router};
use sea_orm::DatabaseConnection;

pub fn create_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/user", post(create_user))
        .with_state(db)
}
