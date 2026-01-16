#[derive(Debug, serde::Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
}
