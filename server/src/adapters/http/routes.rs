use axum::{
    Router,
    routing::{get, post},
};

use crate::adapters::http::handlers;
use crate::app::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(handlers::create_user))
        .route("/users", get(handlers::get_users))
        .route("/users/{:id}", get(handlers::get_user_by_id))
}
