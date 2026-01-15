use async_trait::async_trait;
use sqlx::PgPool;
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

#[async_trait]
impl UserRepo for PostgresUserRepo {
    async fn save(&self, user: &User) -> anyhow::Result<User> {
        let rows = sqlx::query!(
            r#"
            INSERT INTO users DEFAULT VALUES
            RETURNING *
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        match rows.first() {
            Some(r) => Ok(User { id: r.id }),
            None => Err(anyhow::anyhow!(
                "Could not create user with id {}",
                user.id.to_string()
            )),
        }
    }

    async fn get_all(&self) -> anyhow::Result<Vec<User>> {
        let rows = sqlx::query!(
            r#"
            SELECT id
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| User { id: r.id }).collect())
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<User> {
        let row = sqlx::query!(
            r#"
            SELECT id
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(User { id: r.id }),
            None => Err(anyhow::anyhow!("User with id {} not found", id)),
        }
    }
}
