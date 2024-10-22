use std::collections::HashMap;

use http::{header, StatusCode};
use reqwest::{Client, Error};
use serde::de::DeserializeOwned;

pub struct DoApiError {
    pub status: StatusCode,
    pub error: String,
}

impl DoApiError {
    pub fn new(error: Error) -> Self {
        let status = error.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        Self {
            status,
            error: error.to_string(),
        }
    }

    pub fn message(message: String) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: message,
        }
    }

    pub fn custom(status: StatusCode, message: String) -> Self {
        Self {
            status,
            error: message,
        }
    }
}

pub async fn api<T>(
    url: String,
    method: http::Method,
    body: Option<serde_json::Value>,
    form: Option<HashMap<String, String>>,
    authorization: Option<String>,
    basic_auth: Option<(String, Option<String>)>,
    content_type: Option<String>,
) -> Result<T, DoApiError>
where
    T: DeserializeOwned,
{
    let mut headers = header::HeaderMap::new();

    if Option::is_some(&authorization) {
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("{}", authorization.unwrap_or("".to_string())))
                .map_err(|e| DoApiError::message(e.to_string()))?,
        );
    }

    if let Some(content_type) = content_type {
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_str(content_type.as_str())
                .map_err(|e| DoApiError::message(e.to_string()))?,
        );
    }
    let client = Client::new();

    let mut request = client.request(method, &url).headers(headers);

    if let Some(body) = body {
        request = request.json(&body);
    }

    if let Some(form) = form {
        request = request.form(&form)
    }

    if let Some((username, password)) = basic_auth {
        request = request.basic_auth(username, password);
    }

    let response = request.send().await.map_err(|e| DoApiError::new(e))?;

    let is_response_status = response.status().is_success();

    let response_text = response.text().await.map_err(|e| DoApiError::new(e))?;

    if !is_response_status {
        return Err(DoApiError::message(format!("API error: {}", response_text)));
    }

    let response_data = serde_json::from_str(&response_text)
        .map_err(|e| DoApiError::message(format!("JSON parsing error: {}", e)))?;

    Ok(response_data)
}
