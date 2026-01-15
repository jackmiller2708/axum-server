use crate::{domain::user::User, ports::user_repo::UserRepo};
use async_trait::async_trait;
use std::sync::Mutex;

#[derive(Default)]
pub struct MemoryUserRepo {
    users: Mutex<Vec<User>>,
}

#[async_trait]
impl UserRepo for MemoryUserRepo {
    async fn save(&self, user: &User) -> anyhow::Result<User> {
        self.users.lock().unwrap().push(user.clone());
        Ok(user.to_owned())
    }

    async fn get_all(&self) -> anyhow::Result<Vec<User>> {
        let users = self.users.lock().unwrap().clone();
        Ok(users)
    }

    async fn get_by_id(&self, id: u64) -> anyhow::Result<User> {
        let users = self.users.lock().unwrap().clone();
        let user = users.into_iter().find(|user| user.id.is_some() && user.id.unwrap() == id);
        Ok(user.unwrap())
    }
}
