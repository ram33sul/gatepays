use axum::{
    extract::State,
    headers::{authorization::Basic, Authorization},
    Json, TypedHeader,
};
use http::StatusCode;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::{
    helpers::handler_helper::ErrorResponse,
    models::customers::Model,
    services::{customer_service::create_customer, merchant_service::authorize_and_fetch_merchant},
};

#[derive(Serialize, Deserialize)]
pub struct PostCustomerPayload {
    name: String,
    email: String,
    phone_country_code: String,
    phone: String,
    address_id: i32,
    description: String,
}

pub async fn post_customer(
    State(db): State<DatabaseConnection>,
    TypedHeader(authorization): TypedHeader<Authorization<Basic>>,
    Json(payload): Json<PostCustomerPayload>,
) -> Result<Json<Model>, (StatusCode, Json<ErrorResponse>)> {
    let merchant = authorize_and_fetch_merchant(&db, authorization).await?;
    let customer = create_customer(
        &db,
        merchant.id,
        merchant.user_id,
        payload.name,
        payload.email,
        payload.phone_country_code,
        payload.phone,
        payload.address_id,
        payload.description,
    )
    .await
    .map_err(|e| (e.status, Json(ErrorResponse::new(&e.error))))?;
    Ok(Json(customer))
}
