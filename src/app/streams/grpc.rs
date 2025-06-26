use flux_messages_api::{
    streams_service_server::StreamsService, GetLastStreamsRequest, GetLastStreamsResponse,
    GetStreamsRequest, GetStreamsResponse, GetUserStreamsRequest, GetUserStreamsResponse,
};
use tonic::{Request, Response, Status};

use super::service;
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
        req: Request<GetStreamsRequest>,
    ) -> Result<Response<GetStreamsResponse>, Status> {
        let response = get_streams(&self.state, req.into_inner()).await?;

        Ok(Response::new(response))
    }

    async fn get_last_streams(
        &self,
        req: Request<GetLastStreamsRequest>,
    ) -> Result<Response<GetLastStreamsResponse>, Status> {
        let response = get_last_streams(&self.state, req.into_inner()).await?;

        Ok(Response::new(response))
    }

    async fn get_user_streams(
        &self,
        req: Request<GetUserStreamsRequest>,
    ) -> Result<Response<GetUserStreamsResponse>, Status> {
        let response = get_user_streams(&self.state, req.into_inner()).await?;

        Ok(Response::new(response))
    }
}

async fn get_streams(
    AppState { db, .. }: &AppState,
    req: GetStreamsRequest,
) -> Result<GetStreamsResponse, AppError> {
    let res = service::get_streams(db, req.try_into()?).await?;

    Ok(res.into())
}

mod get_streams {
    use anyhow::Error;
    use flux_messages_api::{GetStreamsRequest, GetStreamsResponse};
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        streams::{
            repo::{stream, stream_user},
            service::get_streams::{Req, Res},
        },
    };

    impl TryFrom<GetStreamsRequest> for Req {
        type Error = AppError;

        fn try_from(req: GetStreamsRequest) -> Result<Self, Self::Error> {
            let data = Self {
                stream_ids: req
                    .stream_ids
                    .iter()
                    .map(|id| -> Result<Uuid, Error> { Ok(Uuid::parse_str(id)?) })
                    .collect::<Result<Vec<Uuid>, Error>>()?,
            };

            Ok(data)
        }
    }

    impl From<Res> for GetStreamsResponse {
        fn from(response: Res) -> Self {
            Self {
                streams: response
                    .streams
                    .into_iter()
                    .rev()
                    .map(|(stream, streams_users)| U(stream, streams_users).into())
                    .collect(),
            }
        }
    }

    struct U(stream::Model, Vec<stream_user::Model>);

    impl From<U> for flux_messages_api::get_streams_response::Stream {
        fn from(U(stream, streams_users): U) -> Self {
            Self {
                stream_id: Some(stream.id.into()),
                message_id: Some(stream.message_id.into()),
                text: Some(stream.text),
                user_ids: streams_users.iter().map(|m| m.user_id.into()).collect(),
                messages_count: Some(stream.messages_count),
            }
        }
    }
}

async fn get_last_streams(
    AppState { db, .. }: &AppState,
    req: GetLastStreamsRequest,
) -> Result<GetLastStreamsResponse, AppError> {
    let res = service::get_last_streams(db, req.try_into()?).await?;

    Ok(res.into())
}

mod get_last_streams {
    use std::str::FromStr;

    use flux_lib::locale::Locale;
    use flux_messages_api::{GetLastStreamsRequest, GetLastStreamsResponse};
    use itertools::Itertools as _;

    use crate::app::{
        error::AppError,
        streams::service::get_last_streams::{Request, Response},
    };

    impl From<Response> for GetLastStreamsResponse {
        fn from(req: Response) -> Self {
            Self {
                stream_ids: req.stream_ids.into_iter().map_into().collect(),
            }
        }
    }

    impl TryFrom<GetLastStreamsRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetLastStreamsRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                locale: Locale::from_str(req.locale())?,
            })
        }
    }
}

async fn get_user_streams(
    AppState { db, .. }: &AppState,
    req: GetUserStreamsRequest,
) -> Result<GetUserStreamsResponse, AppError> {
    let res = service::get_user_streams(db, req.try_into()?).await?;

    Ok(res.into())
}

mod get_user_streams {
    use flux_messages_api::{GetUserStreamsRequest, GetUserStreamsResponse};
    use itertools::Itertools;

    use crate::app::{
        error::AppError,
        streams::service::get_user_streams::{Req, Res},
    };

    impl TryFrom<GetUserStreamsRequest> for Req {
        type Error = AppError;

        fn try_from(req: GetUserStreamsRequest) -> Result<Self, Self::Error> {
            let data = Self {
                user_id: req.user_id().try_into()?,
            };

            Ok(data)
        }
    }

    impl From<Res> for GetUserStreamsResponse {
        fn from(res: Res) -> Self {
            Self {
                stream_ids: res.stream_ids.into_iter().map_into().collect(),
            }
        }
    }
}
