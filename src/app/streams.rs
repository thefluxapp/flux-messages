// use tonic::client::GrpcService;

use flux_core_pb::streams_server::StreamsServer;
use grpc::StreamsService;

mod grpc;

pub fn service() -> StreamsServer<StreamsService> {
  StreamsServer::new(StreamsService::default())
}
