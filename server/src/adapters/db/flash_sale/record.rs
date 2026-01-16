use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::flash_sale::FlashSale;

#[derive(Debug, FromRow)]
pub struct FlashSaleRecord {
    pub id: Uuid,
    pub product_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub total_inventory: i32,
    pub remaining_inventory: i32,
    pub per_user_limit: i32,
    pub created_at: DateTime<Utc>,
}

impl FlashSaleRecord {
    pub fn new(
        id: Uuid,
        product_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        total_inventory: i32,
        remaining_inventory: i32,
        per_user_limit: i32,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            product_id,
            start_time,
            end_time,
            total_inventory,
            remaining_inventory,
            per_user_limit,
            created_at,
        }
    }
}

impl TryFrom<FlashSaleRecord> for FlashSale {
    type Error = anyhow::Error;

    fn try_from(r: FlashSaleRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            id: r.id,
            product_id: r.product_id,
            start_time: r.start_time,
            end_time: r.end_time,
            total_inventory: r.total_inventory.try_into()?,
            remaining_inventory: r.remaining_inventory.try_into()?,
            per_user_limit: r.per_user_limit.try_into()?,
            created_at: r.created_at,
        })
    }
}

impl From<&FlashSale> for FlashSaleRecord {
    fn from(f: &FlashSale) -> Self {
        Self {
            id: f.id,
            product_id: f.product_id,
            start_time: f.start_time,
            end_time: f.end_time,
            total_inventory: f.total_inventory.try_into().unwrap(),
            remaining_inventory: f.remaining_inventory.try_into().unwrap(),
            per_user_limit: f.per_user_limit.try_into().unwrap(),
            created_at: f.created_at,
        }
    }
}
