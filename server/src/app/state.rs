use std::sync::Arc;

use crate::ports::user_repo::UserRepo;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<dyn UserRepo>,
}
