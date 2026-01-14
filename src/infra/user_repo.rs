#[derive(Clone)]
pub struct UserRepo;

impl UserRepo {
    pub fn new() -> Self {
        Self
    }

    pub async fn fetch_user(&self) -> String {
        "Jane Doe".to_string()
    }
}
