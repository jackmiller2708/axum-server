use axum::{Router, routing::get};

use crate::{
    http::handlers::{find_all, get_user, health_check},
    http::middleware::logging::logging,
    state::AppState,
};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(find_all))
        .route("/health", get(health_check))
        .route("/users", get(get_user))
        .layer(logging())
        .with_state(state)
}
