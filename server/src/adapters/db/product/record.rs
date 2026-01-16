use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::product::Product;

#[derive(Debug, FromRow)]
pub struct ProductRecord {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl ProductRecord {
    pub fn new(id: Uuid, name: &str, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            name: name.to_string(),
            created_at,
        }
    }
}

impl From<Product> for ProductRecord {
    fn from(p: Product) -> Self {
        Self {
            id: p.id,
            name: p.name.clone(),
            created_at: p.created_at,
        }
    }
}
