use axum::{extract::State, Json};
use http::StatusCode;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::{
    helpers::handler_helper::ErrorResponse, models::order::Model,
    services::stripe_service::create_payment_intent,
};

#[derive(Serialize, Deserialize)]
pub struct PostPaymentIntentPayload {
    amount: i32,
    currency: String,
}

pub async fn post_payment_intent(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<PostPaymentIntentPayload>,
) -> Result<Json<Model>, (StatusCode, Json<ErrorResponse>)> {
    let created_data = create_payment_intent(db, payload.amount, payload.currency)
        .await
        .map_err(|e| (e.status, Json(ErrorResponse::new(&e.error))))?;
    Ok(Json(created_data))
}
