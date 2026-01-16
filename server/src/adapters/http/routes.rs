use axum::{
    Router,
    routing::{get, post},
};

use crate::adapters::http::handler::{product, user};
use crate::app::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(user::create_user))
        .route("/users", get(user::get_users))
        .route("/users/{id}", get(user::get_user_by_id))
        .route("/products", post(product::create_product))
        .route("/products", get(product::get_products))
}
