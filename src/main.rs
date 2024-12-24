mod database;
mod entities;
mod list;
mod migrator;

use clap::{Args, Parser, Subcommand};
use cursive::view::{Nameable, SizeConstraint};
use cursive::views::{
    DummyView, LinearLayout, NamedView, ResizedView, ScrollView, SelectView, TextView,
};
use entities::artists;
use futures::executor::block_on;
use std::process::exit;

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
    let conn = block_on(database::init_database())
        .unwrap_or_else(|err| panic!("Error connecting to database: {err}"));
    match args.command {
        Commands::List(ListEntityArgs { limit, entity }) => {
            println!("{:?}", limit);
            match entity {
                ListEntity::Artists => {
                    let artists = block_on(list::list_artists(&conn, limit))
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

    let artists = block_on(list::list_artists(&conn, None))
        .unwrap_or_else(|err| panic!("Error listing artists: {err}"));
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
