use anyhow::Error;
use async_nats::jetstream::consumer::pull::Config;
use flux_messages_api::streams_service_server::StreamsServiceServer;
use grpc::GrpcStreamsService;

use super::state::AppState;

mod grpc;
mod messaging;
mod repo;
mod service;
pub(super) mod settings;

pub fn streams_service(state: AppState) -> StreamsServiceServer<GrpcStreamsService> {
    StreamsServiceServer::new(GrpcStreamsService::new(state))
}

pub async fn messaging(state: &AppState) -> Result<(), Error> {
    let AppState { js, settings, .. } = state;

    let consumer = js
        .create_consumer_on_stream(
            Config {
                durable_name: Some(settings.streams.messaging.consumer.clone()),
                filter_subject: settings.streams.messaging.subjects.response.clone(),
                ..Default::default()
            },
            settings.streams.messaging.name.clone(),
        )
        .await?;

    // Консюмер молча падает в момент иниализации , а приложка грузится дальше,
    // нужно найти способ пропагейтить ошибку в main тред чтобы приложка не поднималась
    tokio::spawn(messaging::summarize_stream_consumer(
        state.clone(),
        consumer.clone(),
    ));

    Ok(())
}
