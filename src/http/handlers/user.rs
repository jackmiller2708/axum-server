use crate::state::AppState;
use axum::extract::State;

pub async fn get_user(State(state): State<AppState>) -> String {
    state.user_service.get_user().await
}
