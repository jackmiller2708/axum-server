use crate::{
    domain::user::{Email, User},
    ports::user_repo::UserRepo,
};

pub async fn create_user<R: UserRepo + ?Sized>(repo: &R, email: String) -> anyhow::Result<User> {
    let email = Email::parse(&email).map_err(|e| anyhow::anyhow!(e))?;
    let user = User::new(1, email);

    repo.save(&user).await?;

    Ok(user)
}

pub async fn get_users<R: UserRepo + ?Sized>(repo: &R) -> anyhow::Result<Vec<User>> {
    let users = repo.get_all().await?;
    Ok(users)
}

pub async fn get_user_by_id<R: UserRepo + ?Sized>(repo: &R, id: u64) -> anyhow::Result<User> {
    let user = repo.get_by_id(id).await?;
    Ok(user)
}
