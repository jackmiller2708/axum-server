use crate::{
    adapters::{db::memory::MemoryUserRepo, http::router::http_router},
    app::{config::Config, state::AppState},
};
use std::sync::Arc;

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env()?;
    let user_repo = Arc::new(MemoryUserRepo::default());
    let state = AppState { user_repo };
    let app = http_router(state);
    let listener = tokio::net::TcpListener::bind(&config.http_addr).await?;

    tracing::info!("Server listening on {}", config.http_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
