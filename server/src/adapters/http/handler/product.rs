use axum::{Json, extract::State};

use crate::{
    adapters::http::dto::product_reponse::ProductResponse,
    app::state::AppState,
    logic::product::{command::CreateProductCommand, use_case},
};

pub async fn create_product(
    State(state): State<AppState>,
    Json(command): Json<CreateProductCommand>,
) -> Result<Json<ProductResponse>, (axum::http::StatusCode, String)> {
    let product = use_case::save_product(&*state.product_repo, &command)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(product.into()))
}

pub async fn get_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductResponse>>, (axum::http::StatusCode, String)> {
    let products = use_case::get_products(&*state.product_repo)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(products.into_iter().map(|p| p.into()).collect()))
}
