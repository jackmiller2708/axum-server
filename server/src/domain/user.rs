#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<u64>,
    pub email: Email,
}

#[derive(Debug, Clone)]
pub struct Email(pub String);

impl Email {
    pub fn parse(raw: &str) -> Result<Self, &'static str> {
        if raw.contains('@') {
            Ok(Self(raw.to_owned()))
        } else {
            Err("invalid email")
        }
    }
}

impl User {
    pub fn new(email: Email) -> Self {
        Self { id: None, email }
    }
}
