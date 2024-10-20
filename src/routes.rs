use axum::Router;
use connector_routes::connector_routes;
use customer_routes::customer_routes;
use merchant_routes::merchant_routes;
use order_routes::order_routes;
use sea_orm::DatabaseConnection;
use user_routes::user_routes;
mod connector_routes;
mod customer_routes;
mod merchant_routes;
mod order_routes;
mod user_routes;

pub fn create_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .nest("/user", user_routes())
        .nest("/merchant", merchant_routes())
        .nest("/connector", connector_routes())
        .nest("/customer", customer_routes())
        .nest("/order", order_routes())
        .with_state(db)
}
