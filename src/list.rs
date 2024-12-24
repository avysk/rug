use crate::entities::{prelude::*, *};

use sea_orm::*;

pub async fn list_artists(
    conn: &DatabaseConnection,
    limit: Option<u64>,
) -> Result<Vec<artists::Model>, DbErr> {
    Artists::find().limit(limit).all(conn).await
}

pub async fn list_albums(
    conn: &DatabaseConnection,
    limit: Option<u64>,
) -> Result<Vec<albums::Model>, DbErr> {
    Albums::find().limit(limit).all(conn).await
}

pub async fn list_listening_events(
    conn: &DatabaseConnection,
    limit: Option<u64>,
) -> Result<Vec<listening_events::Model>, DbErr> {
    ListeningEvents::find().limit(limit).all(conn).await
}
