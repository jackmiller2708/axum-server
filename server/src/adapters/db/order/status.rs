use crate::domain::order::OrderStatus;

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
