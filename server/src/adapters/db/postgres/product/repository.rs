use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    adapters::db::postgres::product::record::ProductRecord, domain::product::Product,
    ports::product_repo::ProductRepo,
};

pub struct PostgresProductRepo {
    pool: PgPool,
}

impl PostgresProductRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepo for PostgresProductRepo {
    async fn save(&self, product: &Product) -> anyhow::Result<Product> {
        let record: ProductRecord = product.into();
        let saved = sqlx::query_as!(
            ProductRecord,
            r#"
            INSERT INTO products (name, created_at)
            VALUES ($1, $2)
            RETURNING id, name, created_at
            "#,
            record.name,
            record.created_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(saved.try_into()?)
    }

    async fn get_all(&self) -> anyhow::Result<Vec<Product>> {
        let records = sqlx::query_as!(
            ProductRecord,
            r#"
            SELECT id, name, created_at
            FROM products
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let products: Result<Vec<Product>, _> =
            records.into_iter().map(Product::try_from).collect();

        Ok(products?)
    }
}
