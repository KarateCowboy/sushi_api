use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Region::Table)
                    .if_not_exists()
                    .col(pk_auto(Region::Id))
                    .col(string(Region::Slug))
                    .col(string(Region::Katakana))
                    .col(string(Region::English))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Region::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Region {
    Table,
    Id,
    Slug,
    Katakana,
    English
}
