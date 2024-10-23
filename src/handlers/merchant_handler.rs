use axum::{
    extract::{State, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    Json,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::{
    dto::{failure_dto::FailureDto, result_dto::ResultDto},
    helpers::user_helper::verify_jwt,
    models::merchant::Model,
    services::merchant_service::{create_merchant, enable_or_disable_merchant},
};

#[derive(Serialize, Deserialize)]
pub struct PostMerchantPayload {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostMerchantHeader {
    authorization: String,
}

pub async fn post_merchant(
    State(db): State<DatabaseConnection>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<PostMerchantPayload>,
) -> ResultDto<Json<Model>> {
    let validated_data = verify_jwt(authorization.token()).map_err(|e| FailureDto::from(e))?;
    let merchant = create_merchant(db, validated_data.user_id, payload.name).await?;
    Ok(Json(merchant))
}

#[derive(Serialize, Deserialize)]
pub struct ToggleMerchantPayload {
    merchant_id: i32,
    is_enabled: bool,
}

pub async fn toggle_merchant(
    State(db): State<DatabaseConnection>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<ToggleMerchantPayload>,
) -> ResultDto<Json<Model>> {
    let validated_data = verify_jwt(authorization.token()).map_err(|e| FailureDto::from(e))?;
    let merchant = enable_or_disable_merchant(
        &db,
        payload.merchant_id,
        payload.is_enabled,
        validated_data.user_id,
    )
    .await?;
    Ok(Json(merchant))
}
