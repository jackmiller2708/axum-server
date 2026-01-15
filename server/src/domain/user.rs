use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
}

impl User {
    pub fn new(id: Option<Uuid>) -> Self {
        Self {
            id: id.unwrap_or(Uuid::new_v4()),
        }
    }
}
