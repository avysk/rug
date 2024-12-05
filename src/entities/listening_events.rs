//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "listening_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub time: Option<DateTime>,
    #[sea_orm(column_type = "Float")]
    pub rating: f32,
    pub album_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::albums::Entity",
        from = "Column::AlbumId",
        to = "super::albums::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Albums,
}

impl Related<super::albums::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Albums.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
