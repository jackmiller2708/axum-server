use crate::{app::state::AppState, logic::user_logic};
use anyhow::Context;
use axum::{Json, extract::State};
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: String,
}

pub async fn create_user(
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, (axum::http::StatusCode, String)> {
    let user = user_logic::create_user(&*state.user_repo)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
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
                id: user.id.to_string(),
            })
            .collect(),
    ))
}

pub async fn get_user_by_id(
    State(state): State<AppState>,
    params: axum::extract::Path<String>,
) -> Result<Json<UserResponse>, (axum::http::StatusCode, String)> {
    let uuid = Uuid::parse_str(&params.0).context("Invalid UUID").unwrap();
    let user = user_logic::get_user_by_id(&*state.user_repo, uuid)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
    }))
}
