use axum::Router;
use paypal_routes::paypal_routes;
use sea_orm::DatabaseConnection;
use stripe_routes::stripe_routes;
use user_routes::user_routes;
mod paypal_routes;
mod stripe_routes;
mod user_routes;

pub fn create_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .nest("/user", user_routes())
        .nest("/paypal", paypal_routes())
        .nest("/stripe", stripe_routes())
        .with_state(db)
}
