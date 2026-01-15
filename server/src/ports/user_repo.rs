use crate::domain::user::User;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn create(&self) -> anyhow::Result<User>;
    async fn save(&self, user: &User) -> anyhow::Result<User>;
    async fn get_all(&self) -> anyhow::Result<Vec<User>>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<User>;
}
