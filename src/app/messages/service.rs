use anyhow::Error;
use chrono::Utc;
use sea_orm::{DbConn, TransactionTrait as _};
use uuid::Uuid;
use validator::Validate;

use crate::app::error::AppError;

use super::repo;

pub async fn create(db: &DbConn, request: CreateRequest) -> Result<CreateResponse, Error> {
    let txn = db.begin().await?;

    let message = repo::create_message(
        &txn,
        repo::message::Model {
            id: Uuid::now_v7(),
            text: request.text.clone(),
            user_id: request.user_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        },
    )
    .await?;

    if let Some(message_id) = request.message_id {
        repo::find_message_by_id(&txn, message_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let stream = repo::create_stream(
            &txn,
            repo::stream::Model {
                id: Uuid::now_v7(),
                title: None,
                text: None,
                message_id,
                is_main: false,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            },
        )
        .await?
        .ok_or(AppError::NotFound)?;

        repo::create_message_stream(
            &txn,
            repo::message_stream::Model {
                id: Uuid::now_v7(),
                message_id,
                stream_id: stream.id,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            },
        )
        .await?;

        repo::create_message_stream(
            &txn,
            repo::message_stream::Model {
                id: Uuid::now_v7(),
                message_id: message.id,
                stream_id: stream.id,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            },
        )
        .await?;

        repo::create_stream_user(
            &txn,
            repo::stream_user::Model {
                id: Uuid::now_v7(),
                stream_id: stream.id,
                user_id: request.user_id,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            },
        )
        .await?;
    }

    txn.commit().await?;

    Ok(CreateResponse {
        message_id: Uuid::now_v7(),
    })
}

#[derive(Validate)]
pub struct CreateRequest {
    pub text: String,
    pub user_id: Uuid,
    pub message_id: Option<Uuid>,
}
pub struct CreateResponse {
    pub message_id: Uuid,
}
