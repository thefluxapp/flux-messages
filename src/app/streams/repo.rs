use anyhow::Error;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect, Set,
};
use uuid::Uuid;

pub mod message_stream;
pub mod stream;

pub async fn find_stream_by_message_id<T: ConnectionTrait>(
    db: &T,
    message_id: Uuid,
) -> Result<Option<stream::Model>, Error> {
    let stream = stream::Entity::find()
        .filter(stream::Column::MessageId.eq(message_id))
        .one(db)
        .await?;

    Ok(stream)
}

pub async fn find_messages_by_stream_id<T: ConnectionTrait>(
    db: &T,
    stream_id: Uuid,
) -> Result<Vec<message_stream::Model>, Error> {
    Ok(message_stream::Entity::find()
        .filter(message_stream::Column::StreamId.eq(stream_id))
        .all(db)
        .await?)
}

pub async fn find_streams<T: ConnectionTrait>(db: &T) -> Result<Vec<stream::Model>, Error> {
    let streams = stream::Entity::find().all(db).await?;

    Ok(streams)
}

pub async fn update_stream_text<T: ConnectionTrait>(
    db: &T,
    stream_id: Uuid,
    text: String,
    version: DateTime<Utc>,
) -> Result<(), Error> {
    if let Some(stream) = stream::Entity::find_by_id(stream_id)
        .filter(stream::Column::UpdatedAt.lt(version))
        .lock_exclusive()
        .one(db)
        .await?
    {
        let mut stream: stream::ActiveModel = stream.into();
        stream.text = Set(Some(text));
        stream.update(db).await?;
    };

    Ok(())
}
