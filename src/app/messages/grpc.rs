use flux_core_api::{messages_service_server::MessagesService, CreateRequest, CreateResponse};
use tonic::{Request, Response, Status};

use crate::app::state::AppState;

pub struct GrpcMessagesService {
    pub state: AppState,
}

impl GrpcMessagesService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl MessagesService for GrpcMessagesService {
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        // let response = join(&self.state, request.into_inner()).await?;

        Ok(Response::new(CreateResponse {
            message_id: Some("QQQQ".into()),
        }))
    }
}
