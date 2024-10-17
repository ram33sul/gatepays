use axum::{
    headers::{authorization::Basic, Authorization},
    Json,
};
use http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::{
    helpers::{
        api_helper::DoApiError, api_key_helper::generate_api_key, handler_helper::ErrorResponse,
    },
    models::merchant::{self, ActiveModel, Entity as Merchant, Model},
};

pub async fn create_merchant(
    db: DatabaseConnection,
    user_id: i32,
    name: String,
) -> Result<Model, DoApiError> {
    let api_keys = generate_api_key();
    let merchant = ActiveModel {
        user_id: Set(user_id),
        name: Set(name),
        api_key: Set(api_keys.key),
        api_secret: Set(api_keys.secret),
        created_by: Set(user_id),
        ..Default::default()
    };
    let created_merchant = merchant
        .insert(&db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?;
    Ok(created_merchant)
}

pub async fn get_merchant_using_keys(
    db: &DatabaseConnection,
    api_key: String,
    api_secret: String,
) -> Result<Option<Model>, DoApiError> {
    let merchant = Merchant::find()
        .filter(
            Condition::all()
                .add(merchant::Column::ApiKey.eq(&api_key))
                .add(merchant::Column::ApiSecret.eq(&api_secret))
                .add(merchant::Column::IsActive.eq(true)),
        )
        .one(db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?;
    Ok(merchant)
}

pub async fn enable_or_disable_merchant(
    db: &DatabaseConnection,
    merchant_id: i32,
    is_enabled: bool,
    user_id: i32,
) -> Result<Model, DoApiError> {
    let updated_merchant = ActiveModel {
        id: Set(merchant_id),
        is_enabled: Set(is_enabled),
        updated_by: Set(Some(user_id)),
        ..Default::default()
    };
    let merchant = Merchant::update(updated_merchant)
        .exec(db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?;
    Ok(merchant)
}

pub async fn authorize_and_fetch_merchant(
    db: &DatabaseConnection,
    authorization: Authorization<Basic>,
) -> Result<Model, (StatusCode, Json<ErrorResponse>)> {
    let merchant = get_merchant_using_keys(
        db,
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
    Ok(merchant)
}
