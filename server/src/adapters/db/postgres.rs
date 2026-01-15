use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::{domain::user::User, ports::user_repo::UserRepo};

pub struct PostgresUserRepo {
    pool: PgPool,
}

impl PostgresUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, FromRow)]
pub struct UserRecord {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl From<UserRecord> for User {
    fn from(r: UserRecord) -> Self {
        Self {
            id: r.id,
            created_at: r.created_at,
        }
    }
}

#[async_trait]
impl UserRepo for PostgresUserRepo {
    async fn create(&self) -> anyhow::Result<User> {
        let record = sqlx::query_as!(
            UserRecord,
            r#"
            INSERT INTO users DEFAULT VALUES
            RETURNING *
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.into())
    }

    async fn save(&self, user: &User) -> anyhow::Result<User> {
        let record: UserRecord = sqlx::query_as!(
            UserRecord,
            r#"
            INSERT INTO users (id, created_at)
            VALUES ($1, $2)
            RETURNING *
            "#,
            user.id,
            user.created_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.into())
    }

    async fn get_all(&self) -> anyhow::Result<Vec<User>> {
        let rows = sqlx::query_as!(
            UserRecord,
            r#"
            SELECT id, created_at
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<User> {
        let row = sqlx::query_as!(
            UserRecord,
            r#"
            SELECT id, created_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(r.into()),
            None => Err(anyhow::anyhow!("User with id {} not found", id)),
        }
    }
}
