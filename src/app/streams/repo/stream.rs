use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "streams")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub text: Option<String>,
    pub message_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::message_stream::Entity")]
    MessagesStreams,
}

impl Related<super::message_stream::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessagesStreams.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
