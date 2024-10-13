use crate::handlers::user_handler::{create_user, do_login, get_user, get_users};
use axum::{
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;

pub fn user_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", post(create_user))
        .route("/:user_id", get(get_user))
        .route("/list", get(get_users))
        .route("/login", post(do_login))
}
