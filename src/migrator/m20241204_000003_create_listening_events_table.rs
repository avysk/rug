use crate::migrator::m20241204_000002_create_albums_table::Albums;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ListeningEvents::Table)
                    .if_not_exists()
                    .col(pk_auto(ListeningEvents::Id))
                    .col(ColumnDef::new(ListeningEvents::Date).date_time())
                    .col(ColumnDef::new(ListeningEvents::Rating).float().not_null())
                    .col(
                        ColumnDef::new(ListeningEvents::AlbumId)
                            .unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-album-artist_id")
                            .from(ListeningEvents::Table, ListeningEvents::AlbumId)
                            .to(Albums::Table, Albums::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ListeningEvents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ListeningEvents {
    Table,
    Id,
    AlbumId,
    Date, // can be NULL in database; means "unknown"
    Rating,
}
