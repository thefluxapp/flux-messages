use flux_messages_api::{
    messages_service_server::MessagesService, CreateMessageRequest, CreateMessageResponse,
    GetMessageRequest, GetMessageResponse,
};
use tonic::{Request, Response, Status};

use crate::app::{error::AppError, state::AppState};

use super::service;

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
    async fn create_message(
        &self,
        request: Request<CreateMessageRequest>,
    ) -> Result<Response<CreateMessageResponse>, Status> {
        let response = create_message(&self.state, request.into_inner()).await?;

        Ok(Response::new(response.into()))
    }

    async fn get_message(
        &self,
        request: Request<GetMessageRequest>,
    ) -> Result<Response<GetMessageResponse>, Status> {
        let response = get_message(&self.state, request.into_inner()).await?;

        Ok(Response::new(response.into()))
    }
}

async fn get_message(
    AppState { db, settings, .. }: &AppState,
    request: GetMessageRequest,
) -> Result<service::get_message::Response, AppError> {
    let response = service::get_message(&db, &settings.messages, request.try_into()?).await?;

    Ok(response)
}

mod get_message {
    use flux_messages_api::{get_message_response::Message, GetMessageRequest, GetMessageResponse};
    use prost_types::Timestamp;
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        messages::{
            repo::{message, stream},
            service::get_message::{Request, Response},
        },
    };

    impl TryFrom<GetMessageRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetMessageRequest) -> Result<Self, Self::Error> {
            let message_id = Uuid::parse_str(req.message_id())?;

            let cursor_message_id = req
                .cursor_message_id
                .as_deref()
                .map(Uuid::parse_str)
                .transpose()?;

            Ok(Self {
                message_id,
                cursor_message_id,
                limit: req.limit,
            })
        }
    }

    impl From<Response> for GetMessageResponse {
        fn from(res: Response) -> Self {
            Self {
                message: Some(M(res.message.0, res.message.1).into()),
                messages: res
                    .messages
                    .into_iter()
                    .map(|message| M(message.0, message.1).into())
                    .collect(),
                cursor_message_id: if let Some(cursor_message) = res.cursor_message {
                    Some(cursor_message.0.id.into())
                } else {
                    None
                },
            }
        }
    }

    struct M(message::Model, Option<stream::Model>);

    impl From<M> for Message {
        fn from(M(message, stream): M) -> Self {
            Self {
                message_id: Some(message.id.to_string()),
                user_id: Some(message.user_id.to_string()),
                text: Some(message.text),
                code: Some(message.code),
                stream_id: match stream {
                    Some(stream) => Some(stream.id.to_string()),
                    None => None,
                },
                order: Some(message.created_at.and_utc().timestamp_micros()),
                created_at: Some(Timestamp {
                    seconds: message.created_at.and_utc().timestamp(),
                    nanos: 0,
                }),
                updated_at: Some(Timestamp {
                    seconds: message.updated_at.and_utc().timestamp(),
                    nanos: 0,
                }),
            }
        }
    }
}

async fn create_message(
    state: &AppState,
    request: CreateMessageRequest,
) -> Result<service::create_message::Response, AppError> {
    let res = service::create_message(&state.db, request.try_into()?).await?;

    create_message::notify(state, res.clone())?;

    Ok(res)
}

mod create_message {
    use std::str::FromStr;

    use flux_lib::locale::Locale;
    use flux_messages_api::{CreateMessageRequest, CreateMessageResponse};
    use uuid::Uuid;
    use validator::Validate as _;

    use crate::app::{
        error::AppError,
        messages::service::{self, create_message, notify_message},
        state::AppState,
    };

    pub fn notify(state: &AppState, res: create_message::Response) -> Result<(), AppError> {
        tokio::spawn(service::notify_message(state.clone(), res.into()));

        Ok(())
    }

    impl From<create_message::Response> for notify_message::Request {
        fn from(res: create_message::Response) -> Self {
            Self {
                message: res.message,
                stream: res.stream,
            }
        }
    }

    impl TryFrom<CreateMessageRequest> for service::create_message::Request {
        type Error = AppError;

        fn try_from(request: CreateMessageRequest) -> Result<Self, Self::Error> {
            let data = Self {
                text: request.text().into(),
                code: request.code().into(),
                user_id: Uuid::parse_str(request.user_id())?,
                message_id: match request.message_id.clone() {
                    Some(message_id) => Some(Uuid::parse_str(&message_id)?),
                    None => None,
                },
                locale: Locale::from_str(request.locale())?,
            };
            data.validate()?;

            Ok(data)
        }
    }

    impl From<service::create_message::Response> for CreateMessageResponse {
        fn from(val: service::create_message::Response) -> Self {
            Self {
                message_id: Some(val.message.id.into()),
            }
        }
    }
}
