mod m20241204_000001_create_artists_table;
mod m20241204_000002_create_albums_table;

use sea_orm_migration::prelude::*;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241204_000001_create_artists_table::Migration),
            Box::new(m20241204_000002_create_albums_table::Migration),
        ]
    }
}
