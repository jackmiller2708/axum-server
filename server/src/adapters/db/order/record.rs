use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{adapters::db::order::status::OrderStatusDb, domain::order::Order};

#[derive(Debug)]
pub struct OrderRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub flash_sale_id: Uuid,
    pub quantity: i32,
    pub status: OrderStatusDb,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<OrderRecord> for Order {
    type Error = anyhow::Error;

    fn try_from(r: OrderRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            id: r.id,
            user_id: r.user_id,
            flash_sale_id: r.flash_sale_id,
            quantity: r.quantity.try_into()?,
            status: r.status.into(),
            created_at: r.created_at,
        })
    }
}

impl From<&Order> for OrderRecord {
    fn from(o: &Order) -> Self {
        Self {
            id: o.id,
            user_id: o.user_id,
            flash_sale_id: o.flash_sale_id,
            quantity: o.quantity as i32,
            status: o.status.into(),
            created_at: o.created_at,
        }
    }
}
