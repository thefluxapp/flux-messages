use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "streams")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub text: String,
    pub message_id: Uuid,
    pub is_main: bool,
    pub locale: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {
//     #[sea_orm(
//         belongs_to = "super::user::Entity",
//         from = "Column::UserId",
//         to = "super::user::Column::Id"
//     )]
//     User,
//     #[sea_orm(has_many = "super::stream_user::Entity")]
//     StreamsUsers,
//     #[sea_orm(has_many = "super::message_stream::Entity")]
//     MessagesStreams,
//     #[sea_orm(
//         belongs_to = "super::message::Entity",
//         from = "Column::MessageId",
//         to = "super::message::Column::Id"
//     )]
//     Message,
// }

// impl Related<super::message::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Message.def()
//     }
// }

// impl Related<super::user::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::User.def()
//     }
// }

// impl Related<super::message_stream::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::MessagesStreams.def()
//     }
// }

// impl Related<super::stream_user::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::StreamsUsers.def()
//     }
// }

// // TODO: DRY for all models
// #[async_trait::async_trait]
// impl ActiveModelBehavior for ActiveModel {
//     async fn before_save<C>(mut self, _: &C, insert: bool) -> Result<Self, DbErr>
//     where
//         C: ConnectionTrait,
//     {
//         if self.is_not_set(Column::Id) && insert {
//             self.id = Set(Uuid::now_v7());
//             self.created_at = Set(Utc::now().naive_utc());
//         }

//         Ok(self)
//     }
// }
