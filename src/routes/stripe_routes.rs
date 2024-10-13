use axum::{routing::post, Router};
use sea_orm::DatabaseConnection;

use crate::handlers::stripe_handler::post_payment_intent;

pub fn stripe_routes() -> Router<DatabaseConnection> {
    Router::new().route("/payment_intent", post(post_payment_intent))
}
