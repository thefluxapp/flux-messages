use sea_orm_migration::prelude::*;

use crate::{
    m20240927_153528_create_messages::Messages, m20240927_154643_create_streams::Streams,
    m20240927_160951_create_messages_streams::MessagesStreams,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("messages_idx_id")
                    .table(Messages::Table)
                    .col(Messages::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("streams_idx_id")
                    .table(Streams::Table)
                    .col(Streams::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("messages_streams_idx_message_id")
                    .table(MessagesStreams::Table)
                    .col(MessagesStreams::MessageId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("messages_streams_idx_stream_id")
                    .table(MessagesStreams::Table)
                    .col(MessagesStreams::StreamId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("messages_idx_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("streams_idx_id").to_owned())
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("messages_streams_idx_message_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("messages_streams_idx_stream_id")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
