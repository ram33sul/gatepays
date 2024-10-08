use crate::{
    helpers::user_helper::{hash_password, verify_password},
    models::user::{self, ActiveModel, Entity as User},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
    QuerySelect, Set,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserPayload {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    page: u64,
    page_size: u64,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    username_or_email: String,
    password: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl ErrorResponse {
    fn new(message: &str) -> ErrorResponse {
        ErrorResponse {
            message: String::from(message),
        }
    }
}

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<(StatusCode, Json<user::Model>), (StatusCode, Json<ErrorResponse>)> {
    let hashed_password = hash_password(&payload.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new("Cannot hash password")),
        )
    })?;
    let existing_user = User::find()
        .filter(
            Condition::any()
                .add(user::Column::Username.eq(&payload.username))
                .add(user::Column::Email.eq(&payload.email)),
        )
        .one(&db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("Error fetching existing User")),
            )
        })?;
    if let Some(_) = existing_user {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("User already exist")),
        ));
    }
    let user = ActiveModel {
        username: Set(payload.username),
        email: Set(payload.email),
        password: Set(hashed_password),
        created_by: Set(1),
        ..Default::default()
    };

    let created_user = user.insert(&db).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new("Error creating user")),
        )
    })?;

    Ok((StatusCode::CREATED, Json(created_user)))
}

pub async fn get_user(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i32>,
) -> Result<Json<user::Model>, StatusCode> {
    let user = User::find_by_id(user_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(user))
}

pub async fn get_users(
    State(db): State<DatabaseConnection>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<user::Model>>, StatusCode> {
    let users = User::find()
        .offset(params.page * params.page_size)
        .limit(params.page_size)
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(users))
}

pub async fn do_login(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<user::Model>, StatusCode> {
    let user = User::find()
        .filter(
            Condition::any()
                .add(user::Column::Username.eq(&payload.username_or_email))
                .add(user::Column::Email.eq(&payload.username_or_email)),
        )
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    let is_verified = verify_password(&user.password, &payload.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if is_verified {
        Ok(Json(user))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
