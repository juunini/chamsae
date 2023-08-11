//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use super::sea_orm_active_enums::Visibility;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub reply_id: Option<Uuid>,
    pub text: String,
    pub title: Option<String>,
    pub user_id: Option<Uuid>,
    pub visibility: Visibility,
    pub is_sensitive: bool,
    #[sea_orm(unique)]
    pub uri: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ReplyId",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    SelfRef,
    #[sea_orm(has_many = "super::reaction::Entity")]
    Reaction,
    #[sea_orm(has_many = "super::remote_file::Entity")]
    RemoteFile,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::reaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reaction.def()
    }
}

impl Related<super::remote_file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RemoteFile.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
