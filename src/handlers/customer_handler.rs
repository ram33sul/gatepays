use axum::{
    extract::{Path, Query, State},
    headers::{authorization::Basic, Authorization},
    Json, TypedHeader,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::{
    dto::{failure_dto::FailureDto, result_dto::ResultDto},
    models::customers::Model,
    services::{
        customer_service::{
            create_customer, fetch_customer_list, fetch_customer_required, remove_customer,
            update_customer,
        },
        merchant_service::authorize_and_fetch_merchant,
    },
    utils::pagination::PaginationParams,
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
) -> ResultDto<Json<Model>> {
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
    .await?;
    Ok(Json(customer))
}

pub async fn get_customer(
    State(db): State<DatabaseConnection>,
    TypedHeader(authorization): TypedHeader<Authorization<Basic>>,
    Path(customer_id): Path<i32>,
) -> ResultDto<Json<Model>> {
    let merchant = authorize_and_fetch_merchant(&db, authorization).await?;
    let customer = fetch_customer_required(&db, customer_id).await?;
    if customer.merchant_id != merchant.id {
        return Err(FailureDto::unauthorized(
            "Not authorized to access the data".to_string(),
        ));
    }
    Ok(Json(customer))
}

pub async fn get_customer_list(
    State(db): State<DatabaseConnection>,
    TypedHeader(authorization): TypedHeader<Authorization<Basic>>,
    Query(params): Query<PaginationParams>,
) -> ResultDto<Json<Vec<Model>>> {
    let merchant = authorize_and_fetch_merchant(&db, authorization).await?;
    let customers = fetch_customer_list(&db, merchant.id, params.page, params.page_size).await?;
    Ok(Json(customers))
}

pub async fn delete_customer(
    State(db): State<DatabaseConnection>,
    TypedHeader(authorization): TypedHeader<Authorization<Basic>>,
    Path(customer_id): Path<i32>,
) -> ResultDto<Json<bool>> {
    let merchant = authorize_and_fetch_merchant(&db, authorization).await?;
    let customer = fetch_customer_required(&db, customer_id).await?;
    if customer.merchant_id != merchant.id {
        return Err(FailureDto::unauthorized(
            "Not authorized to access the data".to_string(),
        ));
    }
    let is_removed = remove_customer(&db, customer_id).await?;
    Ok(Json(is_removed))
}

#[derive(Serialize, Deserialize)]
pub struct PutCustomerPayload {
    customer_id: i32,
    name: String,
    email: String,
    phone_country_code: String,
    phone: String,
    address_id: i32,
    description: String,
}
pub async fn put_customer(
    State(db): State<DatabaseConnection>,
    TypedHeader(authorization): TypedHeader<Authorization<Basic>>,
    Json(payload): Json<PutCustomerPayload>,
) -> ResultDto<Json<Model>> {
    let merchant = authorize_and_fetch_merchant(&db, authorization).await?;
    let customer = fetch_customer_required(&db, payload.customer_id).await?;
    if customer.merchant_id != merchant.id {
        return Err(FailureDto::unauthorized(
            "Not authorized to access the data".to_string(),
        ));
    }
    let updated_customer = update_customer(
        &db,
        merchant.user_id,
        payload.customer_id,
        payload.name,
        payload.email,
        payload.phone_country_code,
        payload.phone,
        payload.address_id,
        payload.description,
    )
    .await?;
    Ok(Json(updated_customer))
}
