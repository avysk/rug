mod entities;
mod migrator;

use crate::entities::{prelude::*, *};
use crate::migrator::Migrator;
use chrono::Utc;
use futures::executor::block_on;
use platform_dirs::UserDirs;
use sea_orm::*;
use sea_orm_migration::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

const DBNAME: &str = "rug.sqlite";

fn main() {
    if let Err(err) = block_on(init_database()) {
        panic!("Error initializing the database: {}", err);
    }

    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}

fn create_db_path() -> PathBuf {
    let mut db = UserDirs::new().unwrap().document_dir;
    db.push(DBNAME);
    db
}

fn ensure_path(path: &PathBuf) {
    if !path.exists() {
        fs::File::create(path).expect("Couldn't create directory");
    }
}

fn create_db_url(filepath: &Path) -> String {
    let mut db_url = filepath.to_str().unwrap().to_string();
    db_url.insert_str(0, "sqlite://");
    db_url
}

async fn init_database() -> Result<(), DbErr> {
    let db_path = create_db_path();
    ensure_path(&db_path);
    let db_url = create_db_url(&db_path);
    let conn = Database::connect(db_url).await?;

    Migrator::refresh(&conn).await?;

    let foo_artist = artists::ActiveModel {
        name: ActiveValue::Set("Foo".to_owned()),
        note: ActiveValue::Set(Some("A foo artist".to_owned())),
        ..Default::default()
    };
    let res = Artists::insert(foo_artist).exec(&conn).await?;
    let foo_album = albums::ActiveModel {
        title: ActiveValue::Set("Foo Album".to_owned()),
        artist_id: ActiveValue::Set(res.last_insert_id),
        ..Default::default()
    };
    let resalb = Albums::insert(foo_album).exec(&conn).await?;
    let foo_event = listening_events::ActiveModel {
        album_id: ActiveValue::Set(resalb.last_insert_id),
        rating: ActiveValue::Set(4.5),
        time: ActiveValue::Set(Some(Utc::now().naive_utc())),
        ..Default::default()
    };
    ListeningEvents::insert(foo_event).exec(&conn).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_path_ends_with_database_name() {
        let db = create_db_path();
        assert!(db.ends_with(DBNAME));
    }

    #[test]
    fn test_url_starts_with_sqlite_scheme() {
        let db_url = create_db_url(&PathBuf::from("foo"));
        assert!(db_url.starts_with("sqlite://"));
    }
}
