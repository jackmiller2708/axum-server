use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    adapters::db::flash_sale::record::FlashSaleRecord, domain::flash_sale::FlashSale,
    ports::flash_sale_repo::FlashSaleRepo,
};

pub struct PostgresFlashSaleRepo {
    pool: PgPool,
}

impl PostgresFlashSaleRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FlashSaleRepo for PostgresFlashSaleRepo {
    async fn save(&self, flash_sale: &FlashSale) -> anyhow::Result<FlashSale> {
        let record: FlashSaleRecord = flash_sale.into();
        let saved = sqlx::query_as!(
            FlashSaleRecord,
            r#"
            INSERT INTO flash_sales (product_id, start_time, end_time, total_inventory, remaining_inventory, per_user_limit)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, product_id, start_time, end_time, total_inventory, remaining_inventory, per_user_limit, created_at
            "#,
            record.product_id,
            record.start_time,
            record.end_time,
            record.total_inventory,
            record.remaining_inventory,
            record.per_user_limit
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(saved.try_into()?)
    }

    async fn get_all(&self) -> anyhow::Result<Vec<FlashSale>> {
        let records = sqlx::query_as!(
            FlashSaleRecord,
            r#"
            SELECT id, product_id, start_time, end_time, total_inventory, remaining_inventory, per_user_limit, created_at
            FROM flash_sales
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let flash_sales: Result<Vec<FlashSale>, _> =
            records.into_iter().map(FlashSale::try_from).collect();

        Ok(flash_sales?)
    }
}
