use crate::adapters::http::{middleware::logging, routes};
use crate::app::state::AppState;
use axum::Router;

pub fn http_router(state: AppState) -> Router {
    Router::new()
        .merge(routes::routes())
        .layer(logging::logging())
        .with_state(state)
}
