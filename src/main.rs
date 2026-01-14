mod app;
mod http;
mod infra;
mod services;
mod state;

use tracing::info;

#[tokio::main]
async fn main() {
    // Logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting server");

    let app = app::build_app();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!(
        addr = %listener.local_addr().unwrap(),
        "Server listening"
    );

    axum::serve(listener, app).await.unwrap();
}
