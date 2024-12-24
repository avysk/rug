use crate::entities::{prelude::*, *};

use sea_orm::*;

pub async fn list_artists(
    conn: &DatabaseConnection,
    limit: Option<u64>,
) -> Result<Vec<artists::Model>, DbErr> {
    Artists::find().limit(limit).all(conn).await
}
