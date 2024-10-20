use http::Method;
use sea_orm::Set;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    helpers::api_helper::{api, DoApiError},
    models::{connector, gateway, order::ActiveModel},
};

async fn do_api<T>(
    gateway: &gateway::Model,
    endpoint: String,
    method: http::Method,
    body: serde_json::Value,
    authorization: String,
) -> Result<T, DoApiError>
where
    T: DeserializeOwned,
{
    let url = format!("{}/v2{}", gateway.url, endpoint);
    let formatted_authorization = format!("Bearer {}", authorization);

    api(
        url,
        method,
        Some(body),
        None,
        Some(formatted_authorization),
        None,
    )
    .await
}

#[derive(Deserialize, Debug)]
pub struct RefreshAccessTokenResponse {
    // scope: String,
    access_token: String,
    // token_type: String,
    // app_id: String,
    // expires_in: i32,
    // nonce: String,
}
pub async fn refresh_access_token(
    gateway: &gateway::Model,
    connector: &connector::Model,
) -> Result<RefreshAccessTokenResponse, DoApiError> {
    let url = format!(
        "{}/v1/oauth2/token?grant_type=client_credentials",
        gateway.url
    );
    let client_id = &connector.gateway_api_key;
    let secret_key = &connector.gateway_api_secret;
    api(
        url,
        Method::POST,
        None,
        None,
        None,
        Some((client_id.to_string(), Some(secret_key.to_string()))),
    )
    .await
}

#[derive(serde::Serialize)]
pub enum OrderIntent {
    CAPTURE,
    // AUTHORIZE,
}

#[derive(serde::Serialize)]
pub struct OrderAmount {
    currency: String,
    value: String,
}

impl OrderAmount {
    pub fn new(currency: &String, value: &String) -> Self {
        Self {
            currency: currency.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct OrderPurchaseUnit {
    reference_id: String,
    amount: OrderAmount,
}

impl OrderPurchaseUnit {
    pub fn new(reference_id: String, amount: OrderAmount) -> Self {
        Self {
            reference_id,
            amount,
        }
    }
}

#[derive(serde::Serialize)]
pub struct CreateOrderRequest {
    intent: OrderIntent,
    purchase_units: Vec<OrderPurchaseUnit>,
}

impl CreateOrderRequest {
    pub fn new(intent: OrderIntent, purchase_units: Vec<OrderPurchaseUnit>) -> Self {
        Self {
            intent,
            purchase_units,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateOrderResponse {
    pub id: String,
    pub status: OrderStatus,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OrderStatus {
    CREATED,
    SAVED,
    APPROVED,
    VOIDED,
    COMPLETED,
    // PAYER_ACTION_REQUIRED,
}

impl ToString for OrderStatus {
    fn to_string(&self) -> String {
        match self {
            status => format!("{:?}", status),
        }
    }
}

pub async fn create_order(
    gateway: &gateway::Model,
    connector: &connector::Model,
    amount: i32,
    currency: String,
) -> Result<ActiveModel, DoApiError> {
    let access_token: RefreshAccessTokenResponse = refresh_access_token(gateway, connector).await?;
    let body_struct = CreateOrderRequest::new(
        OrderIntent::CAPTURE,
        vec![OrderPurchaseUnit::new(
            String::from("reference_id"),
            OrderAmount::new(&currency, &amount.to_string()),
        )],
    );
    let created_order = do_api::<CreateOrderResponse>(
        gateway,
        String::from("/checkout/orders"),
        Method::POST,
        serde_json::json!(body_struct),
        access_token.access_token,
    )
    .await?;
    let order = ActiveModel {
        gateway_id: Set(gateway.id),
        gateway_order_id: Set((&created_order.id).to_string()),
        status: Set(created_order.status.to_string()),
        amount: Set(amount),
        currency: Set(currency),
        created_by: Set(1),
        ..Default::default()
    };
    Ok(order)
}
