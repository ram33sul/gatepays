use axum::{extract::State, Json};
use http::StatusCode;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::{
    helpers::handler_helper::ErrorResponse,
    models::order::Model,
    services::paypal_service::{create_order, OrderStatus},
};

#[derive(Serialize, Deserialize)]
pub struct PostOrderPayload {
    currency_code: String,
    amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PostOrderResponse {
    id: String,
    status: OrderStatus,
}

pub async fn post_order(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<PostOrderPayload>,
) -> Result<Json<Model>, (StatusCode, Json<ErrorResponse>)> {
    let order_data = create_order(db, payload.amount, payload.currency_code)
        .await
        .map_err(|e| (e.status, Json(ErrorResponse::new(&e.error))))?;
    Ok(Json(order_data))
}
