use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{helpers::api_helper::DoApiError, models::user};

pub async fn fetch_user_for_login(
    db: &DatabaseConnection,
    username_or_email: String,
) -> Result<user::Model, DoApiError> {
    let user = user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Username.eq(&username_or_email))
                .add(user::Column::Email.eq(&username_or_email)),
        )
        .one(db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?
        .ok_or(DoApiError::message("User not found".to_string()))?;
    Ok(user)
}

pub async fn fetch_user(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Option<user::Model>, DoApiError> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?;
    Ok(user)
}

pub async fn fetch_user_required(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<user::Model, DoApiError> {
    let user = fetch_user(db, user_id)
        .await?
        .ok_or(DoApiError::message("User not found".to_string()))?;
    Ok(user)
}
