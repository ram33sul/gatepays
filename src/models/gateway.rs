use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveActiveEnum, EnumIter, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "gateway")]
pub enum Gateway {
    #[sea_orm(string_value = "PAYPAL")]
    PAYPAL,
    #[sea_orm(string_value = "RAZORPAY")]
    RAZORPAY,
    #[sea_orm(string_value = "STRIPE")]
    STRIPE,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "gateways")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: Gateway,
    pub url: String,
    pub is_active: bool,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
