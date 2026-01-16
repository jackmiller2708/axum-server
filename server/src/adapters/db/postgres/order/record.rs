use chrono::{DateTime, Utc};

use uuid::Uuid;

use crate::domain::order::{Order, OrderStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "UPPERCASE")]
pub enum OrderStatusDb {
    Pending,
    Confirmed,
    Failed,
}

impl From<OrderStatusDb> for OrderStatus {
    fn from(s: OrderStatusDb) -> Self {
        match s {
            OrderStatusDb::Pending => OrderStatus::Pending,
            OrderStatusDb::Confirmed => OrderStatus::Confirmed,
            OrderStatusDb::Failed => OrderStatus::Failed,
        }
    }
}

impl From<OrderStatus> for OrderStatusDb {
    fn from(s: OrderStatus) -> Self {
        match s {
            OrderStatus::Pending => OrderStatusDb::Pending,
            OrderStatus::Confirmed => OrderStatusDb::Confirmed,
            OrderStatus::Failed => OrderStatusDb::Failed,
        }
    }
}

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
