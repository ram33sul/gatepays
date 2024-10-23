use axum::{
    extract::State,
    headers::{authorization::Basic, Authorization},
    Json, TypedHeader,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::{
    dto::result_dto::ResultDto,
    models::order::{self},
    services::{merchant_service::authorize_and_fetch_merchant, order_service::create_order},
};

#[derive(Serialize, Deserialize)]
pub struct PostOrderPayload {
    amount: i32,
    currency: String,
    connector_id: Option<i32>,
}

pub async fn post_order(
    State(db): State<DatabaseConnection>,
    TypedHeader(authorization): TypedHeader<Authorization<Basic>>,
    Json(payload): Json<PostOrderPayload>,
) -> ResultDto<Json<order::Model>> {
    let merchant = authorize_and_fetch_merchant(&db, authorization).await?;
    let order = create_order(
        &db,
        merchant.id,
        payload.connector_id,
        payload.amount,
        payload.currency,
    )
    .await?;
    Ok(Json(order))
}
