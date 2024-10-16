use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::{
    helpers::api_helper::DoApiError,
    models::connector::{ActiveModel, Model},
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
