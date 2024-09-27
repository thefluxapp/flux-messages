use flux_core_api::{messages_service_server::MessagesService, CreateRequest, CreateResponse};
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
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let response = create(&self.state, request.into_inner()).await?;

        Ok(Response::new(response))
        // Ok(Response::new(CreateResponse {
        //     message_id: Some("QQQQ".into()),
        // }))
    }
}

async fn create(
    AppState { db, .. }: &AppState,
    request: CreateRequest,
) -> Result<CreateResponse, AppError> {
    let response = service::create(db, request.try_into()?).await?;

    Ok(response.into())
}

impl TryFrom<CreateRequest> for service::CreateRequest {
    type Error = AppError;

    fn try_from(request: CreateRequest) -> Result<Self, Self::Error> {
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

impl Into<CreateResponse> for service::CreateResponse {
    fn into(self) -> CreateResponse {
        CreateResponse {
            message_id: Some(self.message_id.into()),
        }
    }
}
