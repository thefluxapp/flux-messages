use anyhow::Error;
use async_nats::jetstream::consumer::PullConsumer;
use chrono::DateTime;
use flux_messages_api::SummarizeStreamResponse;
use futures_lite::StreamExt as _;
use prost::Message;
use uuid::Uuid;
use validator::ValidationErrors;

use crate::app::{error::AppError, state::AppState, streams::service};

pub async fn summarize_stream_consumer(
    state: AppState,
    consumer: PullConsumer,
) -> Result<(), AppError> {
    loop {
        if let Err(e) = summarize_stream(&state, &consumer).await {
            println!("Error: {}", e)
        }
    }
}

async fn summarize_stream(
    AppState { db, .. }: &AppState,
    consumer: &PullConsumer,
) -> Result<(), Error> {
    let messages = consumer.messages().await?;
    tokio::pin!(messages);

    while let Some(message) = messages.try_next().await? {
        let request = SummarizeStreamResponse::decode(message.payload.clone())?;

        match service::summarize_stream(db, request.try_into()?).await {
            Ok(()) => message.ack().await.map_err(Error::msg)?,
            Err(e) => println!("ErroRR: {}", e),
        };
    }

    Ok(())
}

impl TryFrom<SummarizeStreamResponse> for service::summarize_stream::SummarizeStreamRequest {
    type Error = AppError;

    fn try_from(request: SummarizeStreamResponse) -> Result<Self, Self::Error> {
        let data = Self {
            stream_id: Uuid::parse_str(request.stream_id())
                .map_err(|_| AppError::Validation(ValidationErrors::new()))?,
            text: request.text().into(),
            version: DateTime::from_timestamp_millis(request.version())
                .ok_or(AppError::Validation(ValidationErrors::new()))?,
        };

        Ok(data)
    }
}
