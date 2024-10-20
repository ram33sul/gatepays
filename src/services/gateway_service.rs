use crate::{
    helpers::api_helper::DoApiError,
    models::gateway::{Column, Entity as Gateway, Model},
};
use http::StatusCode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn get_gateway(
    db: &DatabaseConnection,
    gateway_id: i32,
) -> Result<Option<Model>, DoApiError> {
    let gateway = Gateway::find_by_id(gateway_id)
        .filter(Column::IsActive.eq(true))
        .one(db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?;
    Ok(gateway)
}

pub async fn get_gateway_required(
    db: &DatabaseConnection,
    gateway_id: i32,
) -> Result<Model, DoApiError> {
    let gateway = get_gateway(db, gateway_id)
        .await?
        .ok_or(DoApiError::custom(
            StatusCode::BAD_REQUEST,
            "Gateway not found".to_string(),
        ))?;
    Ok(gateway)
}
