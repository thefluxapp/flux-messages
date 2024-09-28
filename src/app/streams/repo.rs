use anyhow::Error;
use sea_orm::{ConnectionTrait, EntityTrait};

pub mod stream;

pub async fn find_all_streams<T: ConnectionTrait>(db: &T) -> Result<Vec<stream::Model>, Error> {
    let streams = stream::Entity::find().all(db).await?;

    Ok(streams)
}
