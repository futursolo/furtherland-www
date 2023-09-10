use fl_www_models::db;
use sea_orm::Schema;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_database_backend();

        let schema = Schema::new(conn);

        manager
            .create_table(schema.create_table_from_entity(db::residents::Entity))
            .await?;
        for stmt in schema.create_index_from_entity(db::residents::Entity) {
            manager.create_index(stmt).await?;
        }

        manager
            .create_table(schema.create_table_from_entity(db::replies::Entity))
            .await?;
        for stmt in schema.create_index_from_entity(db::replies::Entity) {
            manager.create_index(stmt).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(db::replies::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(db::residents::Entity).to_owned())
            .await?;

        Ok(())
    }
}
