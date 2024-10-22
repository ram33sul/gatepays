use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub connector_id: i32,
    pub gateway_order_id: String,
    pub status: String,
    pub amount: i32,
    pub currency: String,
    pub is_active: bool,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
