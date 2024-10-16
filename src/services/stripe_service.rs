use std::collections::HashMap;

use http::Method;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    config,
    helpers::api_helper::{api, DoApiError},
    models::order::{ActiveModel, Model},
};

const STRIPE_GATEWAY_ID: i32 = 3;

async fn do_api<T>(
    endpoint: String,
    method: http::Method,
    form: Option<HashMap<String, String>>,
    secret_key: String,
) -> Result<T, DoApiError>
where
    T: DeserializeOwned,
{
    let env_config = config::Config::from_env();
    let url = format!("{}{}", env_config.stripe_url, endpoint);

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

pub async fn create_payment_intent(
    db: DatabaseConnection,
    amount: i32,
    currency: String,
) -> Result<Model, DoApiError> {
    let env_config = config::Config::from_env();
    let secret_key = env_config.stripe_secret_key;
    let mut form = HashMap::new();
    form.insert("amount".to_string(), amount.to_string());
    form.insert("currency".to_string(), (&currency).to_string());
    let response = do_api::<CreatePaymentResponse>(
        "/payment_intents".to_string(),
        Method::POST,
        Some(form),
        secret_key,
    )
    .await?;
    let order = ActiveModel {
        amount: Set(amount),
        currency: Set(currency),
        gateway_id: Set(STRIPE_GATEWAY_ID),
        gateway_order_id: Set(response.id),
        status: Set(response.status),
        created_by: Set(1),
        ..Default::default()
    };
    let create_order = order
        .insert(&db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?;
    Ok(create_order)
}
