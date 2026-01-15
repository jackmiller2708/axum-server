use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    domain::user::{Email, User},
    ports::user_repo::UserRepo,
};

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
            INSERT INTO users (email)
            VALUES ($1)
            RETURNING *
            "#,
            user.email.0
        )
        .fetch_all(&self.pool)
        .await?;

        match rows.first() {
            Some(r) => Ok(User {
                id: Some(r.id as u64),
                email: Email::parse(&r.email).unwrap()
            }),
            None => Err(anyhow::anyhow!("Could not create user with email {}", user.email.0))
        }
    }

    async fn get_all(&self) -> anyhow::Result<Vec<User>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, email
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| User {
                id: Some(r.id as u64),
                email: Email::parse(&r.email).unwrap(),
            })
            .collect())
    }

    async fn get_by_id(&self, id: u64) -> anyhow::Result<User> {
        let row = sqlx::query!(
            r#"
            SELECT id, email
            FROM users
            WHERE id = $1
            "#,
            id as i64
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(User {
                id: Some(r.id as u64),
                email: Email::parse(&r.email).unwrap(),
            }),
            None => Err(anyhow::anyhow!("User with id {} not found", id)),
        }
    }
}
