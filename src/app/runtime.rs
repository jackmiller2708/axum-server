use crate::{
    adapters::{db::memory::MemoryUserRepo, http::router::http_router},
    app::{config::Config, state::AppState},
};
use std::sync::Arc;

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .init();

    let config = Config::from_env()?;
    tracing::debug!("Configuration loaded: {:?}", config);

    let user_repo = Arc::new(MemoryUserRepo::default());
    tracing::debug!("User repository initialized");

    let state = AppState { user_repo };
    let app = http_router(state);
    tracing::debug!("HTTP router configured");

    let listener = tokio::net::TcpListener::bind(&config.http_addr).await?;

    tracing::info!("Server listening on {}", config.http_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
