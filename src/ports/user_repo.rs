use crate::domain::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn save(&self, user: &User) -> anyhow::Result<()>;
    async fn get_all(&self) -> anyhow::Result<Vec<User>>;
}
