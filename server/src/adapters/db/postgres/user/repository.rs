use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::user::User, ports::user_repo::UserRepo};
use crate::adapters::db::postgres::user::record::UserRecord;

pub struct PostgresUserRepo {
    pool: PgPool,
}

impl PostgresUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
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

        Ok(record.as_user())
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

        Ok(record.as_user())
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

        Ok(rows.into_iter().map(|r| r.as_user()).collect())
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
            Some(r) => Ok(r.as_user()),
            None => Err(anyhow::anyhow!("User with id {} not found", id)),
        }
    }
}
