use sea_orm::entity::prelude::*;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "replies")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: String,
    #[sea_orm(indexed)]
    slug: String,
    approved: bool,
    #[sea_orm(indexed)]
    resident_id: i64,
    content: String,
    lang: Language,
    created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::residents::Entity",
        from = "Column::ResidentId",
        to = "super::residents::Column::Id"
    )]
    Resident,
}

impl Related<super::residents::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Resident.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
