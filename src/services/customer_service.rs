use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set,
};

use crate::{
    dto::{failure_dto::FailureDto, result_dto::ResultDto},
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
) -> ResultDto<Model> {
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
    let created_customer = customer.insert(db).await.map_err(|e| FailureDto::from(e))?;
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
) -> ResultDto<Model> {
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
    let updated_customer = customer.update(db).await.map_err(|e| FailureDto::from(e))?;
    Ok(updated_customer)
}

pub async fn fetch_customer(db: &DatabaseConnection, customer_id: i32) -> ResultDto<Option<Model>> {
    let customer = Customer::find_by_id(customer_id)
        .one(db)
        .await
        .map_err(|e| FailureDto::from(e))?;
    Ok(customer)
}

pub async fn fetch_customer_required(
    db: &DatabaseConnection,
    customer_id: i32,
) -> ResultDto<Model> {
    let customer = fetch_customer(db, customer_id)
        .await?
        .ok_or(FailureDto::from("Customer not found"))?;
    Ok(customer)
}

pub async fn fetch_customer_list(
    db: &DatabaseConnection,
    merchant_id: i32,
    page: u64,
    page_size: u64,
) -> ResultDto<Vec<Model>> {
    let customer = Customer::find()
        .filter(Column::MerchantId.eq(merchant_id))
        .offset(page * page_size)
        .limit(page_size)
        .all(db)
        .await
        .map_err(|e| FailureDto::from(e))?;
    Ok(customer)
}

pub async fn remove_customer(db: &DatabaseConnection, customer_id: i32) -> ResultDto<bool> {
    let customer = ActiveModel {
        id: Set(customer_id),
        is_active: Set(false),
        ..Default::default()
    };
    customer.update(db).await.map_err(|e| FailureDto::from(e))?;
    Ok(true)
}
