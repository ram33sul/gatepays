use std::default;

use http::{header, Method};
use reqwest::{Client, Error, StatusCode};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    config::{self},
    helpers::api_helper::{api, DoApiError},
    models::order::{ActiveModel, Model},
};

const PAYPAL_VENDOR_ID: i32 = 1;

async fn do_api<T>(
    endpoint: String,
    method: http::Method,
    body: serde_json::Value,
    authorization: String,
) -> Result<T, DoApiError>
where
    T: DeserializeOwned,
{
    let env_config = config::Config::from_env();
    let url = format!("{}/v2{}", env_config.paypal_url, endpoint);
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
pub async fn refresh_access_token() -> Result<RefreshAccessTokenResponse, DoApiError> {
    let env_config = config::Config::from_env();
    let url = format!(
        "{}/v1/oauth2/token?grant_type=client_credentials",
        env_config.paypal_url
    );
    let client_id = env_config.paypal_client_id;
    let secret_key = env_config.paypal_secret_key;
    api(
        url,
        Method::POST,
        None,
        None,
        None,
        Some((client_id, Some(secret_key))),
    )
    .await
    // let response = Client::new()
    //     .request(Method::POST, url)
    //     .basic_auth(
    //         env_config.paypal_client_id,
    //         Some(env_config.paypal_secret_key),
    //     )
    //     .send()
    //     .await
    //     .map_err(|e| DoApiError::new(e))?;
    // let token_data = response
    //     .json::<RefreshAccessTokenResponse>()
    //     .await
    //     .map_err(|e| {
    //         println!("{:?}", e);
    //         DoApiError::new(e)
    //     })?;
    // Ok(token_data)
}

#[derive(serde::Serialize)]
pub enum OrderIntent {
    CAPTURE,
    // AUTHORIZE,
}

#[derive(serde::Serialize)]
pub struct OrderAmount {
    currency_code: String,
    value: String,
}

impl OrderAmount {
    pub fn new(currency_code: &String, value: &String) -> Self {
        Self {
            currency_code: currency_code.to_string(),
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
    db: DatabaseConnection,
    amount: i32,
    currency_code: String,
) -> Result<Model, DoApiError> {
    let access_token: RefreshAccessTokenResponse = refresh_access_token().await?;
    let body_struct = CreateOrderRequest::new(
        OrderIntent::CAPTURE,
        vec![OrderPurchaseUnit::new(
            String::from("reference_id"),
            OrderAmount::new(&currency_code, &amount.to_string()),
        )],
    );
    let created_order = do_api::<CreateOrderResponse>(
        String::from("/checkout/orders"),
        Method::POST,
        serde_json::json!(body_struct),
        access_token.access_token,
    )
    .await?;
    let order = ActiveModel {
        vendor_id: Set(PAYPAL_VENDOR_ID),
        vendor_order_id: Set((&created_order.id).to_string()),
        status: Set(created_order.status.to_string()),
        amount: Set(amount),
        currency: Set(currency_code),
        created_by: Set(1),
        ..Default::default()
    };
    let db_order = order
        .insert(&db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?;
    Ok(db_order)
}
