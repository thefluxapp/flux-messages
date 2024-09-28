use anyhow::Error;
use sea_orm::DbConn;

use super::repo;

pub async fn get_streams(db: &DbConn) -> Result<GetStreamsResponse, Error> {
    let streams = repo::find_all_streams(db).await?;

    Ok(GetStreamsResponse { streams })
}

// #[derive(Validate)]
// pub struct GetStreamsRequest {}

pub struct GetStreamsResponse {
    pub streams: Vec<repo::stream::Model>,
}
