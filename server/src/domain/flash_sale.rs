use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct FlashSale {
    pub id: Uuid,
    pub product_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub total_inventory: u32,
    pub remaining_inventory: u32,
    pub per_uer_limit: u8,
    pub created_at: DateTime<Utc>,
}
