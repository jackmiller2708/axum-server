use axum::Router;

use crate::{
    http::routes, infra::user_repo::UserRepo, services::user_service::UserService, state::AppState,
};

pub fn build_app() -> Router {
    // Infra
    let user_repo = UserRepo::new();

    // Services
    let user_service = UserService::new(user_repo);

    // App state (DI container)
    let state = AppState { user_service };

    // HTTP layer
    routes::routes(state)
}
