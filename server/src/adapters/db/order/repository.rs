use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    adapters::db::order::record::OrderRecord, domain::order::Order, ports::order_repo::OrderRepo,
};

pub struct PostgresOrderRepo {
    pool: PgPool,
}

impl PostgresOrderRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrderRepo for PostgresOrderRepo {
    async fn save(&self, order: &Order) -> anyhow::Result<Order> {
        let record: OrderRecord = order.into();
        let saved = sqlx::query_as!(
            OrderRecord,
            r#"
            INSERT INTO orders (user_id, flash_sale_id, quantity, status, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, flash_sale_id, quantity, status as "status: _", created_at
            "#,
            record.user_id,
            record.flash_sale_id,
            record.quantity,
            record.status as _,
            record.created_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(saved.try_into()?)
    }

    async fn get_all(&self) -> anyhow::Result<Vec<Order>> {
        let records = sqlx::query_as!(
            OrderRecord,
            r#"
            SELECT id, user_id, flash_sale_id, quantity, status as "status: _", created_at
            FROM orders
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let orders: Result<Vec<Order>, _> = records.into_iter().map(Order::try_from).collect();

        Ok(orders?)
    }
}
