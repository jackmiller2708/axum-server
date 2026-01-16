use async_trait::async_trait;

use crate::domain::flash_sale::FlashSale;

#[async_trait]
pub trait FlashSaleRepo {
    async fn save(&self, flash_sale: &FlashSale) -> anyhow::Result<FlashSale>;
    async fn get_all(&self) -> anyhow::Result<Vec<FlashSale>>;
}
