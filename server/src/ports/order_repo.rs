use async_trait::async_trait;

use crate::domain::order::Order;

#[async_trait]
pub trait OrderRepo: Send + Sync {
    async fn save(&self, order: &Order) -> anyhow::Result<Order>;
    async fn get_all(&self) -> anyhow::Result<Vec<Order>>;
}
