use crate::user::User;

pub struct Driver {
    user: User,
}

impl Driver {
    pub fn new(name: String, user: String, password: String) -> Self {
        Self {
            user: User::new(name, user, password),
        }
    }
}
