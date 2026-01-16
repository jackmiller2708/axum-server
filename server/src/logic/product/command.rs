use uuid::Uuid;

use crate::adapters::http::dto::product::CreateProductRequest;

#[derive(Debug, Clone)]
pub struct CreateProductCommand {
    pub id: Uuid,
    pub name: String,
}

impl TryFrom<CreateProductRequest> for CreateProductCommand {
    type Error = anyhow::Error;

    fn try_from(value: CreateProductRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            name: value.name,
        })
    }
}
