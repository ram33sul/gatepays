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
    models::connector::Model,
    services::{connector_service::create_connector, merchant_service::get_merchant_using_keys},
};

#[derive(Serialize, Deserialize)]
pub struct PostConnectorPayload {
    merchant_id: i32,
    gateway_id: i32,
    gateway_api_key: String,
    gateway_api_secret: String,
}

pub async fn post_connector(
    State(db): State<DatabaseConnection>,
    TypedHeader(authorization): TypedHeader<Authorization<Basic>>,
    Json(payload): Json<PostConnectorPayload>,
) -> Result<Json<Model>, (StatusCode, Json<ErrorResponse>)> {
    let merchant = get_merchant_using_keys(
        &db,
        authorization.username().to_string(),
        authorization.password().to_string(),
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(&e.error)),
        )
    })?
    .ok_or((
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse::new("Merchant not found")),
    ))?;
    let connector = create_connector(
        db,
        merchant.id,
        payload.gateway_id,
        payload.gateway_api_key,
        payload.gateway_api_secret,
        merchant.user_id,
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(&e.error)),
        )
    })?;
    Ok(Json(connector))
}
