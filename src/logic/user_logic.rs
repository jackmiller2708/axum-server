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
