use crate::{
    domain::product::Product, logic::product::command::CreateProductCommand,
    ports::product_repo::ProductRepo,
};

pub async fn save_product<R: ProductRepo + ?Sized>(
    repo: &R,
    command: CreateProductCommand,
) -> anyhow::Result<Product> {
    repo.save(Product::from(command)).await
}

pub async fn get_products<R: ProductRepo + ?Sized>(repo: &R) -> anyhow::Result<Vec<Product>> {
    repo.get_all().await
}
