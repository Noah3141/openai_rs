//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "meta_completions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub rid: i32,
    pub timestamp: DateTime,
    pub model: String,
    #[sea_orm(column_type = "Double")]
    pub temperature: f64,
    #[sea_orm(column_type = "Text")]
    pub prompt: String,
    #[sea_orm(column_type = "Text")]
    pub query_key: String,
    pub query_key_hash: String,
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
    pub process_time: i32,
    #[sea_orm(column_type = "JsonBinary")]
    pub response: Json,
    #[sea_orm(column_type = "Double")]
    pub cost: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
