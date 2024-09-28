use flux_core_api::{
    get_streams_response::Stream, streams_service_server::StreamsService, GetStreamsRequest,
    GetStreamsResponse,
};
use tonic::{Request, Response, Status};

use crate::app::{error::AppError, state::AppState};

use super::service;

pub struct GrpcStreamsService {
    pub state: AppState,
}

impl GrpcStreamsService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl StreamsService for GrpcStreamsService {
    async fn get_streams(
        &self,
        _: Request<GetStreamsRequest>,
    ) -> Result<Response<GetStreamsResponse>, Status> {
        let response = get_streams(&self.state).await?;

        Ok(Response::new(response))
    }
}

async fn get_streams(AppState { db, .. }: &AppState) -> Result<GetStreamsResponse, AppError> {
    let response = service::get_streams(db).await?;

    Ok(response.into())
}

// impl TryFrom<GetStreamsRequest> for service::GetStreamsRequest {
//     type Error = AppError;

//     fn try_from(request: GetStreamsRequest) -> Result<Self, Self::Error> {
//         let data = Self {};
//         data.validate()?;

//         Ok(data)
//     }
// }

impl Into<GetStreamsResponse> for service::GetStreamsResponse {
    fn into(self) -> GetStreamsResponse {
        GetStreamsResponse {
            streams: self
                .streams
                .iter()
                .map(|stream| Stream {
                    id: Some(stream.id.into()),
                    message_id: Some(stream.message_id.into()),
                    text: Some("QQQ".into()),
                })
                .collect(),
        }
    }
}
