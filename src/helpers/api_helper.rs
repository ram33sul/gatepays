use std::collections::HashMap;

use http::header;
use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::dto::failure_dto::FailureDto;

pub async fn api<T>(
    url: String,
    method: http::Method,
    body: Option<serde_json::Value>,
    form: Option<HashMap<String, String>>,
    authorization: Option<String>,
    basic_auth: Option<(String, Option<String>)>,
    content_type: Option<String>,
) -> Result<T, FailureDto>
where
    T: DeserializeOwned,
{
    let mut headers = header::HeaderMap::new();

    if Option::is_some(&authorization) {
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("{}", authorization.unwrap_or("".to_string())))
                .map_err(|e| FailureDto::from(e))?,
        );
    }

    if let Some(content_type) = content_type {
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_str(content_type.as_str())
                .map_err(|e| FailureDto::from(e))?,
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

    let response = request.send().await.map_err(|e| FailureDto::from(e))?;

    let is_response_status = response.status().is_success();

    let response_text = response.text().await.map_err(|e| FailureDto::from(e))?;

    if !is_response_status {
        return Err(FailureDto::message(format!("API error: {}", response_text)));
    }

    let response_data = serde_json::from_str(&response_text)
        .map_err(|e| FailureDto::message(format!("JSON parsing error: {}", e)))?;

    Ok(response_data)
}
