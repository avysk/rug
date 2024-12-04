use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Artists::Table)
                    .if_not_exists()
                    .col(pk_auto(Artists::Id))
                    .col(ColumnDef::new(Artists::Name).string().not_null())
                    .col(ColumnDef::new(Artists::Note).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Artists::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Artists {
    Table,
    Id,
    Name,
    Note,
}
