use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Failed,
}

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::Pending => "PENDING",
            OrderStatus::Confirmed => "CONFIRMED",
            OrderStatus::Failed => "FAILED",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "PENDING" => Some(OrderStatus::Pending),
            "CONFIRMED" => Some(OrderStatus::Confirmed),
            "FAILED" => Some(OrderStatus::Failed),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub flash_sale_id: Uuid,
    pub quantity: u16,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
}
