use axum::headers::{authorization::Basic, Authorization};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::{
    dto::{failure_dto::FailureDto, result_dto::ResultDto},
    helpers::api_key_helper::generate_api_key,
    models::merchant::{self, ActiveModel, Entity as Merchant, Model},
};

pub async fn create_merchant(
    db: DatabaseConnection,
    user_id: i32,
    name: String,
) -> ResultDto<Model> {
    let api_keys = generate_api_key();
    let merchant = ActiveModel {
        user_id: Set(user_id),
        name: Set(name),
        api_key: Set(api_keys.key),
        api_secret: Set(api_keys.secret),
        created_by: Set(user_id),
        ..Default::default()
    };
    let created_merchant = merchant
        .insert(&db)
        .await
        .map_err(|e| FailureDto::from(e))?;
    Ok(created_merchant)
}

pub async fn get_merchant_using_keys(
    db: &DatabaseConnection,
    api_key: String,
    api_secret: String,
) -> ResultDto<Option<Model>> {
    let merchant: Option<Model> = Merchant::find()
        .filter(
            Condition::all()
                .add(merchant::Column::ApiKey.eq(&api_key))
                .add(merchant::Column::ApiSecret.eq(&api_secret))
                .add(merchant::Column::IsActive.eq(true)),
        )
        .one(db)
        .await
        .map_err(|e| FailureDto::from(e))?;
    Ok(merchant)
}

pub async fn enable_or_disable_merchant(
    db: &DatabaseConnection,
    merchant_id: i32,
    is_enabled: bool,
    user_id: i32,
) -> ResultDto<Model> {
    let updated_merchant = ActiveModel {
        id: Set(merchant_id),
        is_enabled: Set(is_enabled),
        updated_by: Set(Some(user_id)),
        ..Default::default()
    };
    let merchant = Merchant::update(updated_merchant)
        .exec(db)
        .await
        .map_err(|e| FailureDto::from(e))?;
    Ok(merchant)
}

pub async fn authorize_and_fetch_merchant(
    db: &DatabaseConnection,
    authorization: Authorization<Basic>,
) -> ResultDto<Model> {
    let merchant = get_merchant_using_keys(
        db,
        authorization.username().to_string(),
        authorization.password().to_string(),
    )
    .await
    .map_err(|e| e)?
    .ok_or(FailureDto::from("Merchant not found"))?;
    Ok(merchant)
}
