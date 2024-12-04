use crate::migrator::m20241204_000001_create_artists_table::Artists;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Albums::Table)
                    .if_not_exists()
                    .col(pk_auto(Albums::Id))
                    .col(ColumnDef::new(Albums::Title).string().not_null())
                    .col(ColumnDef::new(Albums::Note).string())
                    .col(ColumnDef::new(Albums::ArtistId).unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-album-artist_id")
                            .from(Albums::Table, Albums::ArtistId)
                            .to(Artists::Table, Artists::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Albums::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Albums {
    Table,
    Id,
    ArtistId,
    Title,
    Note,
}
