use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

use crate::{
    adapters::{db::postgres::PostgresUserRepo, http::router::http_router},
    app::{config::Config, state::AppState},
};

pub async fn run() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .init();

    let config = Config::from_env()?;
    tracing::debug!("Configuration loaded: {:?}", config);

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    let user_repo = Arc::new(PostgresUserRepo::new(pool));
    tracing::debug!("User repository initialized");

    let state = AppState { user_repo };
    let app = http_router(state);
    tracing::debug!("HTTP router configured");

    let listener = tokio::net::TcpListener::bind(&config.http_addr).await?;

    tracing::info!("Server listening on {}", config.http_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
