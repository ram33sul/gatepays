use http::Method;
use reqwest::{Body, Client, Error, StatusCode};
use serde::{de::DeserializeOwned, Deserialize};

use crate::config;

struct DoApiError {
    status: StatusCode,
    error: String,
}

impl DoApiError {
    fn new(error: Error) -> Self {
        let status = error.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        Self {
            status,
            error: error.to_string(),
        }
    }

    fn message(message: String) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: message,
        }
    }
}

async fn do_api<T>(endpoint: String, method: http::Method, body: Body) -> Result<T, DoApiError>
where
    T: DeserializeOwned,
{
    let env_config = config::Config::from_env();
    let url = format!("{}{}", env_config.paypal_url, endpoint);
    let client = Client::new();
    let response = client
        .request(method, url)
        .basic_auth(
            env_config.paypal_client_id,
            Some(env_config.paypal_secret_key),
        )
        .body(body)
        .send()
        .await
        .map_err(|e| DoApiError::new(e))?;
    let response_data = response.json::<T>().await.map_err(|e| DoApiError::new(e))?;
    Ok(response_data)
}

#[derive(Deserialize, Debug)]
pub struct RefreshAccessTokenResponse {
    scope: String,
    access_token: String,
    token_type: String,
    app_id: String,
    expires_in: i32,
    nonce: String,
}
pub async fn refresh_access_token() -> Result<RefreshAccessTokenResponse, DoApiError> {
    let body = "{}".to_string();
    let token_data = do_api::<RefreshAccessTokenResponse>(
        String::from("/oauth2/token?grant_type=client_credentials"),
        Method::POST,
        body.into(),
    )
    .await
    .map_err(|e| e)?;
    Ok(token_data)
}

#[derive(serde::Serialize)]
pub enum OrderIntent {
    CAPTURE,
    AUTHORIZE,
}

#[derive(serde::Serialize)]
struct OrderAmount {
    currency_code: String,
    value: String,
}

impl OrderAmount {
    pub fn new(currency_code: String, value: String) -> Self {
        Self {
            currency_code,
            value,
        }
    }
}

#[derive(serde::Serialize)]
struct OrderPurchaseUnit {
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
struct CreateOrderRequest {
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
struct CreateOrderResponse {
    id: String,
    status: OrderStatus,
}

#[derive(Deserialize)]
enum OrderStatus {
    CREATED,
    SAVED,
    APPROVED,
    VOIDED,
    COMPLETED,
    PAYER_ACTION_REQUIRED,
}

pub async fn create_order() -> Result<CreateOrderResponse, DoApiError> {
    let access_token: RefreshAccessTokenResponse = refresh_access_token().await?;
    let body_struct = CreateOrderRequest::new(
        OrderIntent::CAPTURE,
        vec![OrderPurchaseUnit::new(
            String::from("value"),
            OrderAmount::new(String::from("currency_code"), String::from("value")),
        )],
    );
    let body = Body::from(
        serde_json::to_string(&body_struct)
            .map_err(|_| DoApiError::message(String::from("Cannot convert to body")))?,
    );
    let created_order =
        do_api::<CreateOrderResponse>(String::from("/checkout/orders"), Method::POST, body).await?;
    Ok(created_order)
}
