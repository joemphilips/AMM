use sea_orm::entity::prelude::*;
use serde;
use std::convert::TryInto;

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "market")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub name: String,
    pub is_active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::decision::Entity")]
    Decisions,
}

impl Related<super::decision::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Decisions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn get_active_markets() -> Select<Entity> {
        Self::find().filter(Column::IsActive.eq(true))
    }
}
