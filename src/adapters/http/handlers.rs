use crate::{app::state::AppState, logic::user_logic};
use axum::{Json, extract::State};

#[derive(serde::Deserialize)]
pub struct CreateUserDto {
    pub email: String,
}

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub email: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(dto): Json<CreateUserDto>,
) -> Result<Json<UserResponse>, (axum::http::StatusCode, String)> {
    let user = user_logic::create_user(&*state.user_repo, dto.email)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email.0,
    }))
}

pub async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, (axum::http::StatusCode, String)> {
    let users = user_logic::get_users(&*state.user_repo)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(
        users
            .into_iter()
            .map(|user| UserResponse {
                id: user.id,
                email: user.email.0,
            })
            .collect(),
    ))
}
