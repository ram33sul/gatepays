use crate::models::user::{self, ActiveModel};
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserPayload {
    username: String,
    email: String,
    password: String,
}

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<(StatusCode, Json<user::Model>), StatusCode> {
    let user = ActiveModel {
        username: Set(payload.username),
        email: Set(payload.email),
        password: Set(payload.password),
        created_by: Set(1),
        ..Default::default()
    };

    let res = user
        .insert(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(res)))
}
