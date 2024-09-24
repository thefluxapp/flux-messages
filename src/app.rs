use anyhow::Error;
use async_nats::jetstream;
use axum::{routing::get, Router};
use settings::AppSettings;
use state::AppState;
use tonic::service::Routes;

mod settings;
mod state;
mod streams;

pub async fn run() -> Result<(), Error> {
    let settings = AppSettings::new()?;
    let state = AppState::new(settings).await?;

    // messaging(&state).await?;
    // execution(&state).await?;
    http(&state).await?;

    Ok(())
}

async fn http(state: &AppState) -> Result<(), Error> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(flux_core_pb::STREAMS_FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    let router = Router::new()
        .nest("/api", Router::new().route("/healthz", get(|| async {})))
        .with_state(state.to_owned());

    let routes = Routes::from(router);
    let router = routes
        .add_service(reflection_service)
        .add_service(streams::service())
        .into_axum_router();

    let listener = tokio::net::TcpListener::bind(&state.settings.http.endpoint).await?;

    axum::serve(listener, router).await?;

    Ok(())
}

pub type AppJS = jetstream::Context;
