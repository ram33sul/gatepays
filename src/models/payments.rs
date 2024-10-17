use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "payments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub order_id: i32,
    pub status: String,
    pub amount: i32,
    pub amount_received: i32,
    pub net_amount: i32,
    pub currency: String,
    pub payment_method: String,
    pub customer_id: i32,
    pub description: String,
    pub is_active: bool,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
