use crate::infra::user_repo::UserRepo;

#[derive(Clone)]
pub struct UserService {
    repo: UserRepo,
}

impl UserService {
    pub fn new(repo: UserRepo) -> Self {
        Self { repo }
    }

    pub async fn get_user(&self) -> String {
        self.repo.fetch_user().await
    }
}
