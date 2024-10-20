use std::collections::HashMap;

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
    form: Option<HashMap<String, String>>,
    secret_key: String,
) -> Result<T, DoApiError>
where
    T: DeserializeOwned,
{
    let url = format!("{}{}", gateway.url, endpoint);

    api(url, method, None, form, None, Some((secret_key, None))).await
}

#[derive(Serialize)]
pub struct CreatePaymentRequest {
    amount: i32,
    currency: String,
}

impl CreatePaymentRequest {
    pub fn new(amount: i32, currency: String) -> Self {
        Self { amount, currency }
    }
}

#[derive(Deserialize)]
pub struct CreatePaymentResponse {
    id: String,
    status: String,
}

pub async fn create_order(
    gateway: &gateway::Model,
    connector: &connector::Model,
    amount: i32,
    currency: String,
) -> Result<ActiveModel, DoApiError> {
    let secret_key = &connector.gateway_api_secret;
    let mut form = HashMap::new();
    form.insert("amount".to_string(), amount.to_string());
    form.insert("currency".to_string(), (&currency).to_string());
    let response = do_api::<CreatePaymentResponse>(
        gateway,
        "/payment_intents".to_string(),
        Method::POST,
        Some(form),
        secret_key.to_string(),
    )
    .await?;
    let order = ActiveModel {
        amount: Set(amount),
        currency: Set(currency),
        gateway_id: Set(gateway.id),
        gateway_order_id: Set(response.id),
        status: Set(response.status),
        created_by: Set(1),
        ..Default::default()
    };
    Ok(order)
}
