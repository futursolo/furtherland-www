use sea_orm::entity::prelude::*;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "replies")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i64,
    #[sea_orm(indexed)]
    slug: String,
    approved: bool,
    #[sea_orm(indexed)]
    resident_id: u64,
    content: String,
    lang: Language,
    created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
