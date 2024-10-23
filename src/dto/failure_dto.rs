use axum::Json;
use http::{header::InvalidHeaderValue, StatusCode};
use sea_orm::DbErr;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorMessage {
    message: String,
}

impl ErrorMessage {
    pub fn new(message: String) -> ErrorMessage {
        ErrorMessage { message }
    }

    pub fn json(message: String) -> Json<ErrorMessage> {
        Json(ErrorMessage { message })
    }
}

pub struct FailureDto(StatusCode, Json<ErrorMessage>);

impl FailureDto {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self(
            status,
            Json(ErrorMessage {
                message: message.into(),
            }),
        )
    }

    pub fn message(message: String) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    pub fn db_error(error: DbErr) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }

    pub fn hash_error(error: argon2::password_hash::Error) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }

    pub fn jwt_error(error: jsonwebtoken::errors::Error) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, error.to_string())
    }

    pub fn bad_request(message: String) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn internal(message: String) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    pub fn unauthorized(message: String) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
    }
}

impl axum::response::IntoResponse for FailureDto {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}

impl From<DbErr> for FailureDto {
    fn from(error: DbErr) -> Self {
        Self::db_error(error)
    }
}

impl From<argon2::password_hash::Error> for FailureDto {
    fn from(error: argon2::password_hash::Error) -> Self {
        Self::hash_error(error)
    }
}

impl From<jsonwebtoken::errors::Error> for FailureDto {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Self::jwt_error(error)
    }
}

impl From<&str> for FailureDto {
    fn from(message: &str) -> Self {
        Self::bad_request(message.to_string())
    }
}

impl From<InvalidHeaderValue> for FailureDto {
    fn from(error: InvalidHeaderValue) -> Self {
        Self::bad_request(error.to_string())
    }
}

impl From<reqwest::Error> for FailureDto {
    fn from(error: reqwest::Error) -> Self {
        Self::new(
            error.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            error.to_string(),
        )
    }
}
