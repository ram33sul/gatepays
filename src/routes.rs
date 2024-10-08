use axum::Router;
use sea_orm::DatabaseConnection;
use user_routes::user_routes;
mod user_routes;

pub fn create_routes(db: DatabaseConnection) -> Router {
    Router::new().nest("/user", user_routes(db))
}
