use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::user::User;

#[derive(Debug, FromRow)]
pub struct UserRecord {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl UserRecord {
    pub fn as_user(&self) -> User {
        User {
            id: self.id,
            created_at: self.created_at,
        }
    }
}
