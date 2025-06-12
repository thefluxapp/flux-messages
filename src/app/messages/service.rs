use crate::app::{error::AppError, state::AppState};
use anyhow::Error;
use bytes::BytesMut;
use chrono::Utc;
use create_message::{Request, Response};
use prost::Message;
use sea_orm::{DbConn, TransactionTrait as _};
use uuid::Uuid;

use super::{repo, settings::MessagesSettings};

pub async fn get_message(
    db: &DbConn,
    settings: &MessagesSettings,
    req: get_message::Request,
) -> Result<get_message::Response, Error> {
    let message = repo::find_message_by_id(db, req.message_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let stream = repo::find_stream_by_message_id(db, req.message_id).await?;
    let parent_stream = match stream {
        Some(ref stream) => {
            repo::find_parent_stream_by_message_id(db, message.id, stream.id).await?
        }
        None => None,
    };

    let message = (message, parent_stream);
    let limit: u64 = if let Some(limit) = req.limit {
        limit.try_into()?
    } else {
        settings.limit
    };

    let mut messages = match stream {
        Some(stream) => {
            repo::find_messages_by_stream_id(db, stream.id, req.cursor_message_id, limit + 1)
                .await?
        }
        None => vec![message.clone()],
    };

    let cursor_message = if messages.len() > limit.try_into()? {
        Some(messages.remove(0))
    } else {
        None
    };

    Ok(get_message::Response {
        message,
        messages,
        cursor_message,
    })
}

pub mod get_message {
    use uuid::Uuid;

    use crate::app::messages::repo;

    pub struct Request {
        pub message_id: Uuid,
        pub cursor_message_id: Option<Uuid>,
        pub limit: Option<i64>,
    }
    pub struct Response {
        pub message: (repo::message::Model, Option<repo::stream::Model>),
        pub messages: Vec<(repo::message::Model, Option<repo::stream::Model>)>,
        pub cursor_message: Option<(repo::message::Model, Option<repo::stream::Model>)>,
    }
}

pub async fn create_message(db: &DbConn, request: Request) -> Result<Response, Error> {
    let txn = db.begin().await?;

    let message = repo::create_message(
        &txn,
        repo::message::Model {
            id: Uuid::now_v7(),
            text: request.text.clone(),
            user_id: request.user_id,
            code: request.code,
            locale: Some(request.locale.to_string()),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        },
    )
    .await?;

    let stream = match request.message_id {
        Some(message_id) => {
            let parent_message = repo::find_message_by_id(&txn, message_id)
                .await?
                .ok_or(AppError::NotFound)?;

            let is_main = match repo::find_message_stream_by_message_id(&txn, message_id).await? {
                Some(_) => false,
                None => true,
            };

            let stream = repo::create_stream(
                &txn,
                repo::stream::Model {
                    id: Uuid::now_v7(),
                    text: parent_message.text.clone(),
                    message_id,
                    is_main,
                    locale: parent_message.locale,
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

            repo::create_stream_user(
                &txn,
                repo::stream_user::Model {
                    id: Uuid::now_v7(),
                    stream_id: stream.id,
                    user_id: parent_message.user_id,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                },
            )
            .await?;

            Some(stream)
        }
        None => None,
    };

    txn.commit().await?;

    Ok(Response { message, stream })
}

pub mod create_message {
    use flux_lib::locale::Locale;
    use uuid::Uuid;
    use validator::Validate;

    use crate::app::messages::repo::{message, stream};

    #[derive(Validate)]
    pub struct Request {
        pub text: String,
        pub user_id: Uuid,
        pub message_id: Option<Uuid>,
        pub code: String,
        pub locale: Locale,
    }

    #[derive(Clone)]
    pub struct Response {
        pub message: message::Model,
        pub stream: Option<stream::Model>,
    }
}

pub async fn notify_message(state: AppState, req: notify_message::Request) -> Result<(), Error> {
    let AppState { settings, db, js } = state;

    let streams_users = if let Some(stream) = req.stream.clone() {
        repo::find_streams_users_by_stream_id(db.as_ref(), stream.id).await?
    } else {
        vec![]
    };

    let mut buf = BytesMut::new();
    Into::<flux_messages_api::Message>::into(notify_message::M {
        message: req.message,
        stream: req.stream,
        streams_users,
    })
    .encode(&mut buf)?;

    js.publish(settings.messages.messaging.message.subject, buf.into())
        .await?;

    Ok(())
}

pub mod notify_message {
    use flux_messages_api::message::Stream;
    use prost_types::Timestamp;

    use crate::app::messages::repo;

    pub struct Request {
        pub message: repo::message::Model,
        pub stream: Option<repo::stream::Model>,
    }

    pub struct M {
        pub message: repo::message::Model,
        pub stream: Option<repo::stream::Model>,
        pub streams_users: Vec<repo::stream_user::Model>,
    }

    impl From<M> for flux_messages_api::Message {
        fn from(
            M {
                message,
                stream,
                streams_users,
            }: M,
        ) -> Self {
            Self {
                message_id: Some(message.id.into()),
                user_id: Some(message.user_id.into()),
                text: Some(message.text),
                code: Some(message.code),
                order: Some(message.created_at.and_utc().timestamp_micros()),
                created_at: Some(Timestamp {
                    seconds: message.created_at.and_utc().timestamp(),
                    nanos: 0,
                }),
                updated_at: Some(Timestamp {
                    seconds: message.updated_at.and_utc().timestamp(),
                    nanos: 0,
                }),
                stream: match stream {
                    Some(stream) => Some(Stream {
                        stream_id: Some(stream.id.into()),
                        message_id: Some(stream.message_id.into()),
                        user_ids: streams_users.iter().map(|v| v.user_id.into()).collect(),
                    }),
                    None => None,
                },
            }
        }
    }
}
