use anyhow::Error;
use sea_orm::{DbConn, TransactionTrait};
use summarize_stream::SummarizeStreamRequest;

use super::repo;

pub async fn get_last_streams(db: &DbConn) -> Result<get_last_streams::Res, Error> {
    let stream_ids = repo::find_last_streams(db)
        .await?
        .iter()
        .map(|m| m.id)
        .collect();

    Ok(get_last_streams::Res { stream_ids })
}

pub mod get_last_streams {
    use uuid::Uuid;

    pub struct Res {
        pub stream_ids: Vec<Uuid>,
    }
}

pub async fn get_user_streams(
    db: &DbConn,
    req: get_user_streams::Req,
) -> Result<get_user_streams::Res, Error> {
    let stream_ids = repo::find_user_streams_with_streams_users(db, req.user_id)
        .await?
        .iter()
        .map(|m| m.id)
        .collect();

    Ok(get_user_streams::Res { stream_ids })
}

pub mod get_user_streams {
    use uuid::Uuid;

    pub struct Req {
        pub user_id: Uuid,
    }
    pub struct Res {
        pub stream_ids: Vec<Uuid>,
    }
}

pub async fn get_streams(db: &DbConn, req: get_streams::Req) -> Result<get_streams::Res, Error> {
    let streams = repo::find_streams_with_streams_users(db, req.stream_ids).await?;

    Ok(get_streams::Res { streams })
}

pub mod get_streams {
    use uuid::Uuid;

    use crate::app::streams::repo::{stream, stream_user};

    pub struct Req {
        pub stream_ids: Vec<Uuid>,
    }

    pub struct Res {
        pub streams: Vec<(stream::Model, Vec<stream_user::Model>)>,
    }
}

pub async fn summarize_stream(db: &DbConn, request: SummarizeStreamRequest) -> Result<(), Error> {
    let txn = db.begin().await?;

    repo::update_stream_text(&txn, request.stream_id, request.text, request.version).await?;

    txn.commit().await?;

    Ok(())
}

pub mod summarize_stream {
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use uuid::Uuid;

    #[derive(Deserialize, Debug)]
    pub struct SummarizeStreamRequest {
        pub stream_id: Uuid,
        pub text: String,
        pub version: DateTime<Utc>,
    }
}
