use crate::{
    domain::product::Product, logic::product::command::CreateProductCommand,
    ports::product_repo::ProductRepo,
};

pub async fn save_product<R: ProductRepo + ?Sized>(
    repo: &R,
    command: &CreateProductCommand,
) -> anyhow::Result<Product> {
    let product = Product {
        id: uuid::Uuid::new_v4(),
        name: command.name.clone(),
        created_at: chrono::Utc::now(),
    };

    repo.save(&product).await
}

pub async fn get_products<R: ProductRepo + ?Sized>(repo: &R) -> anyhow::Result<Vec<Product>> {
    repo.get_all().await
}
