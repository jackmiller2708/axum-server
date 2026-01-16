use async_trait::async_trait;
use chrono::DateTime;
use std::sync::Mutex;
use uuid::Uuid;

use crate::{domain::user::User, ports::user_repo::UserRepo};

#[derive(Default)]
pub struct MemoryUserRepo {
    users: Mutex<Vec<User>>,
}

#[async_trait]
impl UserRepo for MemoryUserRepo {
    async fn create(&self) -> anyhow::Result<User> {
        let created_user = User {
            id: Uuid::new_v4(),
            created_at: DateTime::default(),
        };

        self.users.lock().unwrap().push(created_user.clone());
        Ok(created_user)
    }

    async fn save(&self, user: &User) -> anyhow::Result<User> {
        self.users.lock().unwrap().push(user.clone());
        Ok(user.to_owned())
    }

    async fn get_all(&self) -> anyhow::Result<Vec<User>> {
        let users = self.users.lock().unwrap().clone();
        Ok(users)
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<User> {
        let users = self.users.lock().unwrap().clone();
        let user = users.into_iter().find(|user| user.id == id);
        Ok(user.unwrap())
    }
}
