//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    ToSchema,
    Default,
)]
#[sea_orm(table_name = "t_key_alias")]
#[schema(as = KeyAliasModel)]
pub struct Model {
    #[sea_orm(column_name = "_id", primary_key)]
    #[serde(skip)]
    pub id: i64,
    pub key_id: String,
    pub alias: String,
    #[serde(skip)]
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTime,
    #[serde(skip)]
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}