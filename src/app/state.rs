use crate::ports::user_repo::UserRepo;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<dyn UserRepo>,
}
