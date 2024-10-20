use anyhow::Error;
use sea_orm::{DbConn, TransactionTrait};
use summarize_stream::SummarizeStreamRequest;

use super::repo;

pub async fn get_stream(
    db: &DbConn,
    request: get_stream::Request,
) -> Result<get_stream::Response, Error> {
    let stream = repo::find_stream_by_message_id(db, request.message_id).await?;
    let messages_streams = match stream {
        Some(ref stream) => repo::find_messages_by_stream_id(db, stream.id).await?,
        None => vec![],
    };

    Ok(get_stream::Response {
        stream,
        messages_streams,
    })
}

pub mod get_stream {
    use uuid::Uuid;
    use validator::Validate;

    use crate::app::streams::repo;

    #[derive(Validate)]
    pub struct Request {
        pub message_id: Uuid,
    }
    #[derive(Debug)]
    pub struct Response {
        pub stream: Option<repo::stream::Model>,
        pub messages_streams: Vec<repo::message_stream::Model>,
    }
}

pub async fn get_streams(db: &DbConn) -> Result<GetStreamsResponse, Error> {
    let streams = repo::find_streams(db).await?;

    Ok(GetStreamsResponse { streams })
}

pub async fn summarize_stream(db: &DbConn, request: SummarizeStreamRequest) -> Result<(), Error> {
    let txn = db.begin().await?;

    repo::update_stream_text(&txn, request.stream_id, request.text, request.version).await?;

    txn.commit().await?;

    Ok(())
}

pub struct GetStreamsResponse {
    pub streams: Vec<repo::stream::Model>,
}

pub mod summarize_stream {
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use uuid::Uuid;

    #[derive(Deserialize, Debug)]
    pub struct SummarizeStreamRequest {
        pub stream_id: Uuid,
        pub text: String,
        pub version: DateTime<Utc>,
    }
}
