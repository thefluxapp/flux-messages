use flux_core_pb::{streams_server::Streams, GetStreamRequest, GetStreamResponse};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct StreamsService {}

#[tonic::async_trait]
impl Streams for StreamsService {
    async fn get_stream(
        &self,
        request: Request<GetStreamRequest>,
    ) -> Result<Response<GetStreamResponse>, Status> {
        let request = request.into_inner();
        let response = GetStreamResponse { bar: request.foo };

        Ok(Response::new(response))
    }
}
