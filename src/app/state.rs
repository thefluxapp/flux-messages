use std::sync::Arc;

use anyhow::Error;
use async_nats::jetstream;

use super::{settings::AppSettings, AppJS};

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    pub _js: Arc<AppJS>,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        let nats = async_nats::connect(&settings.nats.endpoint).await?;
        let js = Arc::new(jetstream::new(nats));

        Ok(Self { settings, _js: js })
    }
}
