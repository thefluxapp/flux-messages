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

pub fn messaging(state: &AppState) {
    tokio::spawn(messaging::stream(state.clone()));
}
