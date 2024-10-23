use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    dto::{failure_dto::FailureDto, result_dto::ResultDto},
    models::user,
};

pub async fn fetch_user_for_login(
    db: &DatabaseConnection,
    username_or_email: String,
) -> ResultDto<user::Model> {
    let user = user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Username.eq(&username_or_email))
                .add(user::Column::Email.eq(&username_or_email)),
        )
        .one(db)
        .await
        .map_err(|e| FailureDto::from(e))?
        .ok_or(FailureDto::from("User not found"))?;
    Ok(user)
}

pub async fn fetch_user(db: &DatabaseConnection, user_id: i32) -> ResultDto<Option<user::Model>> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|e| FailureDto::from(e))?;
    Ok(user)
}

pub async fn fetch_user_required(db: &DatabaseConnection, user_id: i32) -> ResultDto<user::Model> {
    let user = fetch_user(db, user_id)
        .await?
        .ok_or(FailureDto::from("User not found"))?;
    Ok(user)
}
