use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::{
    dto::{failure_dto::FailureDto, result_dto::ResultDto},
    models::connector::{self, ActiveModel, Model},
};

pub async fn create_connector(
    db: DatabaseConnection,
    merchant_id: i32,
    gateway_id: i32,
    gateway_api_key: String,
    gateway_api_secret: String,
    user_id: i32,
) -> ResultDto<Model> {
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
        .map_err(|e| FailureDto::from(e))?;
    Ok(created_connector)
}

pub async fn get_connector_required(
    db: &DatabaseConnection,
    merchant_id: i32,
    connector_id: Option<i32>,
) -> ResultDto<connector::Model> {
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
        .map_err(|e| FailureDto::from(e))?
        .ok_or(FailureDto::from("Connector not found"))?;
    Ok(connector)
}
