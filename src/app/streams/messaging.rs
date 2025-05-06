use anyhow::Error;
use futures_lite::StreamExt as _;
use tracing::error;

use crate::app::state::AppState;

pub async fn stream(state: AppState) -> Result<(), Error> {
    let AppState { js, settings, .. } = state.clone();

    let consumer = stream::consumer(&js, &settings).await?;
    let mut messages = consumer.messages().await?;

    while let Some(message) = messages.next().await {
        if let Err(err) = stream::handler(state.clone(), message?).await {
            error!("{}", err);
        }
    }

    Ok(())
}

mod stream {
    use async_nats::jetstream::{
        self,
        consumer::{pull::Config, Consumer},
    };
    use chrono::DateTime;
    use flux_lib::error::Error;
    use prost::Message as _;
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        settings::AppSettings,
        state::AppState,
        streams::service::{self, messaging_stream::Request},
        AppJS,
    };

    pub async fn consumer(js: &AppJS, settings: &AppSettings) -> Result<Consumer<Config>, Error> {
        Ok(js
            .create_consumer_on_stream(
                Config {
                    durable_name: Some(settings.streams.messaging.stream.consumer.clone()),
                    filter_subjects: settings.streams.messaging.stream.subjects.clone(),
                    ..Default::default()
                },
                settings.nats.stream.clone(),
            )
            .await?)
    }

    pub async fn handler(state: AppState, message: jetstream::Message) -> Result<(), Error> {
        service::messaging_stream(state, message.clone().try_into()?).await?;

        message.ack().await.map_err(Error::msg)?;
        Ok(())
    }

    impl TryFrom<jetstream::Message> for Request {
        type Error = AppError;

        fn try_from(message: jetstream::Message) -> Result<Self, Self::Error> {
            let message = flux_ai_api::Stream::decode(message.payload.clone())?;

            Ok(Self {
                message_id: Uuid::parse_str(message.message_id())?,
                text: message.text().into(),
                updated_at: DateTime::from_timestamp(message.version(), 0)
                    .ok_or(AppError::NotFound)?,
            })
        }
    }
}
