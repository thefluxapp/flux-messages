use flux_core_api::messages_service_server::MessagesServiceServer;
use grpc::GrpcMessagesService;

use super::state::AppState;

mod grpc;

pub fn messages_service(state: AppState) -> MessagesServiceServer<GrpcMessagesService> {
    MessagesServiceServer::new(GrpcMessagesService::new(state))
}
