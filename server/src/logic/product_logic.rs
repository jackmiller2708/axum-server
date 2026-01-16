use uuid::Uuid;

use crate::{domain::product::Product, ports::product_repo::ProductRepo};

pub async fn save_product<R: ProductRepo + ?Sized>(
    repo: &R,
    product: &Product,
) -> anyhow::Result<Product> {
    repo.save(product).await
}

pub async fn get_products<R: ProductRepo + ?Sized>(repo: &R) -> anyhow::Result<Vec<Product>> {
    repo.get_all().await
}
