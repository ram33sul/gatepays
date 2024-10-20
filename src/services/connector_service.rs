use http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::{
    helpers::api_helper::DoApiError,
    models::connector::{self, ActiveModel, Model},
};

pub async fn create_connector(
    db: DatabaseConnection,
    merchant_id: i32,
    gateway_id: i32,
    gateway_api_key: String,
    gateway_api_secret: String,
    user_id: i32,
) -> Result<Model, DoApiError> {
    let connector = ActiveModel {
        merchant_id: Set(merchant_id),
        gateway_id: Set(gateway_id),
        gateway_api_key: Set(gateway_api_key),
        gateway_api_secret: Set(gateway_api_secret),
        created_by: Set(user_id),
        ..Default::default()
    };
    let created_connector = connector
        .insert(&db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?;
    Ok(created_connector)
}

pub async fn get_connector_required(
    db: &DatabaseConnection,
    merchant_id: i32,
    connector_id: Option<i32>,
) -> Result<connector::Model, DoApiError> {
    let connector_filter = Condition::all()
        .add(connector::Column::IsActive.eq(true))
        .add(connector::Column::MerchantId.eq(merchant_id));
    let connector_query = match connector_id {
        Some(connector_id) => connector::Entity::find_by_id(connector_id),
        None => connector::Entity::find().order_by_asc(connector::Column::CreatedAt),
    };
    let connector = connector_query
        .filter(connector_filter)
        .one(db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?
        .ok_or(DoApiError::custom(
            StatusCode::BAD_REQUEST,
            "Connector not found".to_string(),
        ))?;
    Ok(connector)
}
