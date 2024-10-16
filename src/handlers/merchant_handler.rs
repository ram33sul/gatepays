use axum::{
    extract::{State, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    Json,
};
use http::StatusCode;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::{
    helpers::{handler_helper::ErrorResponse, user_helper::verify_jwt},
    models::merchant::Model,
    services::merchant_service::create_merchant,
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
) -> Result<Json<Model>, (StatusCode, Json<ErrorResponse>)> {
    let validated_data = verify_jwt(authorization.token())
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::new("Not authorized")),
            )
        })
        .map_err(|e| e)?;
    let merchant = create_merchant(db, validated_data.user_id, payload.name)
        .await
        .map_err(|e| (e.status, Json(ErrorResponse::new(&e.error))))?;
    Ok(Json(merchant))
}
