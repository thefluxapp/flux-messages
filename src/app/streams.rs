use flux_core_api::streams_service_server::StreamsServiceServer;
use grpc::GrpcStreamsService;

use super::state::AppState;

mod grpc;
mod repo;
mod service;

pub fn streams_service(state: AppState) -> StreamsServiceServer<GrpcStreamsService> {
    StreamsServiceServer::new(GrpcStreamsService::new(state))
}
