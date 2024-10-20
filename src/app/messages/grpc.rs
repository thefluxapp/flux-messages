use anyhow::Error;
use flux_core_api::{
    get_messages_response::Message, messages_service_server::MessagesService, CreateMessageRequest,
    CreateMessageResponse, GetMessagesRequest, GetMessagesResponse,
};
use tonic::{Request, Response, Status};
use uuid::Uuid;
use validator::{Validate as _, ValidationErrors};

use crate::app::{error::AppError, state::AppState};

use super::{repo, service};

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

    async fn get_messages(
        &self,
        request: Request<GetMessagesRequest>,
    ) -> Result<Response<GetMessagesResponse>, Status> {
        let response = get_messages(&self.state, request.into_inner()).await?;

        Ok(Response::new(response.into()))
    }
}

async fn create_message(
    state: &AppState,
    request: CreateMessageRequest,
) -> Result<service::CreateMessageResponse, AppError> {
    let response = service::create_message(&state.db, request.try_into()?).await?;

    if let Some(ref stream) = response.stream {
        tokio::spawn(summarize_stream_by_message_id(
            state.clone(),
            stream.clone(),
        ));
    }

    Ok(response)
}

async fn summarize_stream_by_message_id(
    AppState { settings, db, js }: AppState,
    stream: repo::stream::Model,
) -> Result<(), AppError> {
    service::summarize_stream_by_message_id(&db, &js, settings, stream).await?;

    Ok(())
}

impl TryFrom<CreateMessageRequest> for service::CreateMessageRequest {
    type Error = AppError;

    fn try_from(request: CreateMessageRequest) -> Result<Self, Self::Error> {
        let data = Self {
            text: request.text().into(),
            user_id: Uuid::parse_str(request.user_id())
                .map_err(|_| AppError::Validation(ValidationErrors::new()))?,
            message_id: match request.message_id {
                Some(message_id) => Some(
                    Uuid::parse_str(&message_id)
                        .map_err(|_| AppError::Validation(ValidationErrors::new()))?,
                ),
                None => None,
            },
        };
        data.validate()?;

        Ok(data)
    }
}

impl Into<CreateMessageResponse> for service::CreateMessageResponse {
    fn into(self) -> CreateMessageResponse {
        CreateMessageResponse {
            message_id: Some(self.message.id.into()),
        }
    }
}

async fn get_messages(
    state: &AppState,
    request: GetMessagesRequest,
) -> Result<service::GetMessagesResponse, AppError> {
    let response = service::get_messages(&state.db, request.try_into()?).await?;

    Ok(response)
}

impl TryFrom<GetMessagesRequest> for service::GetMessagesRequest {
    type Error = AppError;

    fn try_from(request: GetMessagesRequest) -> Result<Self, Self::Error> {
        let data = Self {
            message_ids: request
                .message_ids
                .iter()
                .map(|message_id| -> Result<Uuid, Error> { Ok(Uuid::parse_str(message_id)?) })
                .collect::<Result<Vec<Uuid>, Error>>()?,
        };

        Ok(data)
    }
}

impl Into<GetMessagesResponse> for service::GetMessagesResponse {
    fn into(self) -> GetMessagesResponse {
        GetMessagesResponse {
            messages: self
                .messages
                .iter()
                .map(|message| Message {
                    message_id: Some(message.id.into()),
                    user_id: Some(message.user_id.into()),
                    text: Some(message.text.clone()),
                })
                .collect(),
        }
    }
}
