use chrono::{DateTime, Utc};

use crate::domain::order::Order;

pub struct OrderResponse {
    pub id: String,
    pub user_id: String,
    pub flash_sale_id: String,
    pub quantity: u32,
    pub created_at: DateTime<Utc>,
}

impl From<Order> for OrderResponse {
    fn from(order: Order) -> Self {
        Self {
            id: order.id.to_string(),
            user_id: order.user_id.to_string(),
            flash_sale_id: order.flash_sale_id.to_string(),
            quantity: order.quantity as u32,
            created_at: order.created_at,
        }
    }
}
