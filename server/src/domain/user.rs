use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: Uuid, created_at: DateTime<Utc>) -> Self {
        Self { id, created_at }
    }
}
