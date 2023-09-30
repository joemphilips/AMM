use async_graphql::*;
use bitcoin::PublicKey;
use sea_orm::entity::prelude::*;
use serde;
use std::convert::TryInto;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    DeriveEntityModel,
    SimpleObject,
)]
#[sea_orm(table_name = "market")]
#[graphql(concrete(name = "decision", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub is_past: bool,
    pub oracle_public_key: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::market::Entity")]
    Market,
}

impl Related<super::market::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Market.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
