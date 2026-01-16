#[derive(Debug, Clone, serde::Deserialize)]

pub struct CreateProductCommand {
    pub name: String,
}
