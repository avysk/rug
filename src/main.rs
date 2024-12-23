mod entities;
mod migrator;

use crate::entities::{prelude::*, *};
use crate::migrator::Migrator;

use chrono::Utc;
use clap::{Args, Parser, Subcommand};
use cursive::view::{Nameable, SizeConstraint};
use cursive::views::{
    DummyView, LinearLayout, NamedView, ResizedView, ScrollView, SelectView, TextView,
};
use futures::executor::block_on;
use platform_dirs::UserDirs;
use sea_orm::*;
use sea_orm_migration::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

const DBNAME: &str = "rug.sqlite";

#[derive(Debug, Parser)]
#[command(name = "rug")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Adds entities to the database
    Add(EntityArgs),
    /// Lists entities from the database
    List(ListEntityArgs),
    /// Deletes entities from_the database
    Delete(EntityArgs),
}

#[derive(Args, Debug)]
#[command(flatten_help = true)]
struct EntityArgs {
    #[command(subcommand)]
    entity: Entity,
}

#[derive(Args, Debug)]
#[command(flatten_help = true)]
struct ListEntityArgs {
    /// Limit the number of results
    #[arg(short, long)]
    limit: Option<u64>,
    #[command(subcommand)]
    entity: ListEntity,
}

#[derive(Clone, Debug, Subcommand)]
enum ListEntity {
    /// List artists
    Artists,
    /// List albums
    Albums,
    /// List listening events
    Listens,
}

#[derive(Clone, Debug, Subcommand)]
enum Entity {
    /// Operate on artist
    Artist { name: String, note: Option<String> },
    /// Operate on album
    Album {
        artistid: u32,
        title: String,
        note: Option<String>,
    },
    /// Operate on listening event
    Listen { albumid: u32, rating: f64 },
}

fn main() {
    let args = Cli::parse();
    let conn = block_on(init_database())
        .unwrap_or_else(|err| panic!("Error connecting to database: {err}"));
    match args.command {
        Commands::List(ListEntityArgs { limit, entity }) => {
            println!("{:?}", limit);
            match entity {
                ListEntity::Artists => {
                    let artists = block_on(Artists::find().limit(limit).all(&conn))
                        .unwrap_or_else(|err| panic!("Error listing artists: {err}"));
                    for artist in artists.iter() {
                        println!("{:?}", artist);
                    }
                }
                ListEntity::Albums => println!("listing albums"),
                ListEntity::Listens => println!("listing listening events"),
            }
        }
        _ => todo!(),
    }
    exit(0);

    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let artists =
        block_on(list_artists(&conn)).unwrap_or_else(|err| panic!("Error listing artists: {err}"));
    let mut artist_name = ScrollView::new(SelectView::new().with_name("artist name"));
    for artist in artists.iter() {
        artist_name
            .get_inner_mut()
            .get_mut()
            .add_item(artist.name.to_owned(), artist.clone())
    }
    let artist_note = ScrollView::new(TextView::new("").with_name("artist note"));
    let artist_note_scroll = artist_note.with_name("artist name scroll");
    artist_name
        .get_inner_mut()
        .get_mut()
        .set_on_select(|s, artist: &artists::Model| {
            s.call_on_name("artist note", |view: &mut TextView| {
                if let Some(note) = artist.note.to_owned() {
                    view.set_content(note);
                } else {
                    view.set_content("");
                }
            });
            s.call_on_name(
                "artist note scroll",
                |view_scroll: &mut ScrollView<NamedView<TextView>>| {
                    view_scroll.scroll_to_top();
                    view_scroll.scroll_to_left();
                },
            );
        });
    let artists_layer = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Full,
        LinearLayout::horizontal()
            .child(artist_name)
            .child(DummyView::new())
            .child(artist_note_scroll),
    );
    siv.add_layer(artists_layer);

    siv.call_on_name("artist note", |view: &mut TextView| {
        view.set_content("foobar")
    });

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

async fn init_database() -> Result<DatabaseConnection, DbErr> {
    let db_path = create_db_path();
    ensure_path(&db_path);
    let db_url = create_db_url(&db_path);
    let conn = Database::connect(db_url).await?;

    Migrator::up(&conn, None).await?;

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
    Ok(conn)
}

async fn list_artists(conn: &DatabaseConnection) -> Result<Vec<artists::Model>, DbErr> {
    Artists::find().all(conn).await
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
