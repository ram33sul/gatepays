use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveActiveEnum, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "vendor")]
pub enum Vendor {
    #[sea_orm(string_value = "PAYPAL")]
    PAYPAL,
    #[sea_orm(string_value = "RAZORPAY")]
    RAZORPAY,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "vendors")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: Vendor,
    pub is_active: bool,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
