use crate::adapters::http::handlers;
use crate::app::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(handlers::create_user))
        .route("/users", get(handlers::get_users))
}
