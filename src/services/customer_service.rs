use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::{
    helpers::api_helper::DoApiError,
    models::customers::{ActiveModel, Column, Entity as Customer, Model},
};

pub async fn create_customer(
    db: &DatabaseConnection,
    merchant_id: i32,
    user_id: i32,
    name: String,
    email: String,
    phone_country_code: String,
    phone: String,
    address_id: i32,
    description: String,
) -> Result<Model, DoApiError> {
    let customer = ActiveModel {
        merchant_id: Set(merchant_id),
        name: Set(name),
        email: Set(email),
        phone_country_code: Set(phone_country_code),
        phone: Set(phone),
        address_id: Set(address_id),
        description: Set(description),
        created_by: Set(user_id),
        ..Default::default()
    };
    let created_customer = customer
        .insert(db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?;
    Ok(created_customer)
}

pub async fn update_customer(
    db: &DatabaseConnection,
    user_id: i32,
    customer_id: i32,
    name: String,
    email: String,
    phone_country_code: String,
    phone: String,
    address_id: i32,
    description: String,
) -> Result<Model, DoApiError> {
    let customer = ActiveModel {
        id: Set(customer_id),
        name: Set(name),
        email: Set(email),
        phone_country_code: Set(phone_country_code),
        phone: Set(phone),
        address_id: Set(address_id),
        description: Set(description),
        updated_by: Set(Some(user_id)),
        ..Default::default()
    };
    let updated_customer = customer
        .update(db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?;
    Ok(updated_customer)
}

pub async fn retrieve_customer(
    db: &DatabaseConnection,
    customer_id: i32,
) -> Result<Option<Model>, DoApiError> {
    let customer = Customer::find_by_id(customer_id)
        .one(db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?;
    Ok(customer)
}

pub async fn retrieve_customer_list(
    db: &DatabaseConnection,
    merchant_id: i32,
) -> Result<Vec<Model>, DoApiError> {
    let customer = Customer::find()
        .filter(Column::MerchantId.eq(merchant_id))
        .all(db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?;
    Ok(customer)
}

pub async fn delete_customer(
    db: &DatabaseConnection,
    customer_id: i32,
) -> Result<bool, DoApiError> {
    let customer = ActiveModel {
        id: Set(customer_id),
        is_active: Set(false),
        ..Default::default()
    };
    customer
        .update(db)
        .await
        .map_err(|e| DoApiError::message(e.to_string()))?;
    Ok(true)
}
