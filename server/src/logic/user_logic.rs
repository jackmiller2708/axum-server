use uuid::Uuid;

use crate::{
    domain::user::{User},
    ports::user_repo::UserRepo,
};

pub async fn create_user<R: UserRepo + ?Sized>(repo: &R) -> anyhow::Result<User> {
    let created_user = repo.create().await?;

    Ok(created_user)
}

pub async fn get_users<R: UserRepo + ?Sized>(repo: &R) -> anyhow::Result<Vec<User>> {
    let users = repo.get_all().await?;

    Ok(users)
}

pub async fn get_user_by_id<R: UserRepo + ?Sized>(repo: &R, id: Uuid) -> anyhow::Result<User> {
    let user = repo.get_by_id(id).await?;

    Ok(user)
}
