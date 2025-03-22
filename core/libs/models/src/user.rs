#[derive(Debug)]
pub struct User {
    name: String,
    user: String,
    password: String,
}

impl User {
    pub fn new (name: String, user: String, password: String) -> Self {
        Self { name, user, password }
    }
}
