use axum::{
    extract::State,
    headers::{authorization::Basic, Authorization},
    Json, TypedHeader,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::{
    dto::{failure_dto::FailureDto, result_dto::ResultDto},
    models::connector::Model,
    services::{
        connector_service::create_connector, merchant_service::authorize_and_fetch_merchant,
    },
};

#[derive(Serialize, Deserialize)]
pub struct PostConnectorPayload {
    gateway_id: i32,
    gateway_api_key: String,
    gateway_api_secret: String,
}

pub async fn post_connector(
    State(db): State<DatabaseConnection>,
    TypedHeader(authorization): TypedHeader<Authorization<Basic>>,
    Json(payload): Json<PostConnectorPayload>,
) -> ResultDto<Json<Model>> {
    let merchant = authorize_and_fetch_merchant(&db, authorization).await?;
    if merchant.is_enabled == false {
        return Err(FailureDto::from("Merchant is disabled"));
    }
    let connector = create_connector(
        db,
        merchant.id,
        payload.gateway_id,
        payload.gateway_api_key,
        payload.gateway_api_secret,
        merchant.user_id,
    )
    .await?;
    Ok(Json(connector))
}
