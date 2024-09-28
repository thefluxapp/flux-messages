use flux_core_api::{
    messages_service_server::MessagesService, CreateMessageRequest, CreateMessageResponse,
};
use tonic::{Request, Response, Status};
use uuid::Uuid;
use validator::{Validate as _, ValidationErrors};

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
        let response = create(&self.state, request.into_inner()).await?;

        Ok(Response::new(response))
    }
}

async fn create(
    AppState { db, .. }: &AppState,
    request: CreateMessageRequest,
) -> Result<CreateMessageResponse, AppError> {
    let response = service::create(db, request.try_into()?).await?;

    Ok(response.into())
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
            message_id: Some(self.message_id.into()),
        }
    }
}
