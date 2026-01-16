use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
