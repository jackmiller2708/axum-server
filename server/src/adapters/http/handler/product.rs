use axum::{Json, extract::State};

use crate::{
    adapters::http::dto::product_reponse::ProductResponse, app::state::AppState,
    domain::product::Product, logic::product_logic,
};

pub async fn create_product(
    State(state): State<AppState>,
    Json(product): Json<Product>,
) -> Result<Json<ProductResponse>, (axum::http::StatusCode, String)> {
    let product = product_logic::save_product(&*state.product_repo, &product)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(product.into()))
}

pub async fn get_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductResponse>>, (axum::http::StatusCode, String)> {
    let products = product_logic::get_products(&*state.product_repo)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(products.into_iter().map(|p| p.into()).collect()))
}
