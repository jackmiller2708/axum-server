use chrono::{DateTime, Utc};

use crate::domain::flash_sale::FlashSale;

pub struct FlashSaleResponse {
    pub id: String,
    pub product_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub total_inventory: u32,
    pub remaining_inventory: u32,
    pub per_user_limit: u8,
}

impl From<FlashSale> for FlashSaleResponse {
    fn from(flash_sale: FlashSale) -> Self {
        Self {
            id: flash_sale.id.to_string(),
            product_id: flash_sale.product_id.to_string(),
            start_time: flash_sale.start_time,
            end_time: flash_sale.end_time,
            total_inventory: flash_sale.total_inventory,
            remaining_inventory: flash_sale.remaining_inventory,
            per_user_limit: flash_sale.per_user_limit,
        }
    }
}
