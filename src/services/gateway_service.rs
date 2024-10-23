use crate::{
    dto::{failure_dto::FailureDto, result_dto::ResultDto},
    models::gateway::{Column, Entity as Gateway, Model},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn get_gateway(db: &DatabaseConnection, gateway_id: i32) -> ResultDto<Option<Model>> {
    let gateway = Gateway::find_by_id(gateway_id)
        .filter(Column::IsActive.eq(true))
        .one(db)
        .await
        .map_err(|e| FailureDto::from(e))?;
    Ok(gateway)
}

pub async fn get_gateway_required(db: &DatabaseConnection, gateway_id: i32) -> ResultDto<Model> {
    let gateway = get_gateway(db, gateway_id)
        .await?
        .ok_or(FailureDto::from("Gateway not found"))?;
    Ok(gateway)
}
