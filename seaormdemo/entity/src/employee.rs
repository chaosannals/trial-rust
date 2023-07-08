use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// 使用了 migration 的话，表名要对应，sea-orm-cli 工具生成示例名字都对不上。
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "so_employee")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub text: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
