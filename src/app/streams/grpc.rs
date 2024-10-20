use flux_core_api::{
    streams_service_server::StreamsService, GetStreamsRequest, GetStreamsResponse,
};
use tonic::{Request, Response, Status};

use super::{repo, service};
use crate::app::{error::AppError, state::AppState};

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

    // async fn get_stream(
    //     &self,
    //     request: Request<GetStreamRequest>,
    // ) -> Result<Response<GetStreamResponse>, Status> {
    //     let response = get_stream(&self.state, request.into_inner()).await?;

    //     Ok(Response::new(response.into()))
    // }
}

// async fn get_stream(
//     AppState { db, .. }: &AppState,
//     request: GetStreamRequest,
// ) -> Result<GetStreamResponse, AppError> {
//     let response = service::get_stream(db, request.try_into()?).await?;

//     Ok(response.into())
// }

// impl TryFrom<GetStreamRequest> for service::get_stream::Request {
//     type Error = AppError;

//     fn try_from(request: GetStreamRequest) -> Result<Self, Self::Error> {
//         let data = Self {
//             message_id: Uuid::parse_str(request.message_id())
//                 .map_err(|_| AppError::Validation(ValidationErrors::new()))?,
//         };
//         data.validate()?;

//         Ok(data)
//     }
// }

// impl From<service::get_stream::Response> for GetStreamResponse {
//     fn from(response: service::get_stream::Response) -> Self {
//         GetStreamResponse {
//             stream: match response.stream {
//                 Some(stream) => Some(stream.into()),
//                 None => None,
//             },
//             message_ids: response
//                 .messages_streams
//                 .iter()
//                 .map(|m| m.message_id.into())
//                 .collect(),
//         }
//     }
// }

// impl From<repo::stream::Model> for flux_core_api::get_stream_response::Stream {
//     fn from(stream: repo::stream::Model) -> Self {
//         Self {
//             stream_id: Some(stream.id.into()),
//             message_id: Some(stream.message_id.into()),
//             text: stream.text,
//         }
//     }
// }

async fn get_streams(AppState { db, .. }: &AppState) -> Result<GetStreamsResponse, AppError> {
    let response = service::get_streams(db).await?;

    Ok(response.into())
}

impl From<service::GetStreamsResponse> for GetStreamsResponse {
    fn from(response: service::GetStreamsResponse) -> Self {
        GetStreamsResponse {
            streams: response
                .streams
                .into_iter()
                .map(|stream| stream.into())
                .collect(),
        }
    }
}

impl From<repo::stream::Model> for flux_core_api::get_streams_response::Stream {
    fn from(stream: repo::stream::Model) -> Self {
        Self {
            stream_id: Some(stream.id.into()),
            message_id: Some(stream.message_id.into()),
            text: stream.text,
        }
    }
}
