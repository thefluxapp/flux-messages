use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Messages {
    Table,
    Id,
    Text,
    UserId,
    Code,
    Locale,
}

#[derive(DeriveIden)]
pub enum Streams {
    Table,
    Id,
    Title,
    Text,
    MessageId,
    IsMain,
    Locale,
    MessagesCount,
}

#[derive(DeriveIden)]
pub enum MessagesStreams {
    Table,
    Id,
    MessageId,
    StreamId,
}

#[derive(DeriveIden)]
pub enum StreamsUsers {
    Table,
    Id,
    StreamId,
    UserId,
}
