use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::DatabaseConnection;

use crate::handlers::customer_handler::{
    delete_customer, get_customer, get_customer_list, post_customer, put_customer,
};

pub fn customer_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", post(post_customer))
        .route("/:customer_id", get(get_customer))
        .route("/list", get(get_customer_list))
        .route("/", delete(delete_customer))
        .route("/", put(put_customer))
}
