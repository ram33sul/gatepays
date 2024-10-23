use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::dto::failure_dto::FailureDto;
use crate::dto::result_dto::ResultDto;
use crate::gateway_services::{paypal_service, stripe_service};
use crate::models::gateway::Gateway;
use crate::models::order::{self};
use crate::services::gateway_service::get_gateway_required;

use super::connector_service::get_connector_required;

pub async fn create_order(
    db: &DatabaseConnection,
    merchant_id: i32,
    connector_id: Option<i32>,
    amount: i32,
    currency: String,
) -> ResultDto<order::Model> {
    let connector = get_connector_required(db, merchant_id, connector_id).await?;
    let gateway = get_gateway_required(db, connector.gateway_id).await?;
    let order_model: order::ActiveModel;
    if gateway.name == Gateway::PAYPAL {
        order_model = paypal_service::create_order(&gateway, &connector, amount, currency).await?;
    } else if gateway.name == Gateway::STRIPE {
        order_model = stripe_service::create_order(&gateway, &connector, amount, currency).await?;
    } else {
        return Err(FailureDto::from("Invalid Gateway"));
    }
    let order = order_model
        .insert(db)
        .await
        .map_err(|e| FailureDto::from(e))?;
    Ok(order)
}
