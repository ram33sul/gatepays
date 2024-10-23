use crate::{
    dto::failure_dto::FailureDto,
    helpers::user_helper::{hash_password, sign_jwt, verify_password},
    models::user::{self, ActiveModel, Entity as User},
    services::user_service::{fetch_user_for_login, fetch_user_required},
    utils::pagination::PaginationParams,
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
pub struct LoginPayload {
    username_or_email: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthorisedResponse {
    user_data: user::Model,
    token: String,
}

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<(StatusCode, Json<AuthorisedResponse>), FailureDto> {
    let hashed_password = hash_password(&payload.password).map_err(|e| FailureDto::from(e))?;
    let existing_user = User::find()
        .filter(
            Condition::any()
                .add(user::Column::Username.eq(&payload.username))
                .add(user::Column::Email.eq(&payload.email)),
        )
        .one(&db)
        .await
        .map_err(|e| FailureDto::from(e))?;
    if let Some(_) = existing_user {
        return Err(FailureDto::from("User already Exists"));
    }
    let user = ActiveModel {
        username: Set(payload.username),
        email: Set(payload.email),
        password: Set(hashed_password),
        created_by: Set(1),
        ..Default::default()
    };

    let created_user = user.insert(&db).await.map_err(|e| FailureDto::from(e))?;

    let token = sign_jwt(&created_user.id).map_err(|e| FailureDto::from(e))?;

    let data = AuthorisedResponse {
        user_data: created_user,
        token,
    };

    Ok((StatusCode::CREATED, Json(data)))
}

pub async fn get_user(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i32>,
) -> Result<Json<user::Model>, FailureDto> {
    let user = fetch_user_required(&db, user_id).await.map_err(|e| e)?;
    Ok(Json(user))
}

pub async fn get_users(
    State(db): State<DatabaseConnection>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<user::Model>>, FailureDto> {
    let users = User::find()
        .offset(params.page * params.page_size)
        .limit(params.page_size)
        .all(&db)
        .await
        .map_err(|e| FailureDto::from(e))?;
    Ok(Json(users))
}

pub async fn do_login(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthorisedResponse>, FailureDto> {
    let user = fetch_user_for_login(&db, payload.username_or_email).await?;
    let is_verified =
        verify_password(&user.password, &payload.password).map_err(|e| FailureDto::from(e))?;
    if is_verified {
        let token = sign_jwt(&user.id).map_err(|e| FailureDto::from(e))?;
        let data = AuthorisedResponse {
            user_data: user,
            token,
        };
        Ok(Json(data))
    } else {
        Err(FailureDto::unauthorized(
            "Incorrect Credentials".to_string(),
        ))
    }
}
