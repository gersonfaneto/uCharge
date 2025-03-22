use crate::user::User;

pub struct Owner {
    user: User,
    payment_token: String,
}

impl Owner {
    pub fn new(name: String, user: String, password: String, payment_token: String) -> Self {
        Self {
            user: User::new(name, user, password),
            payment_token,
        }
    }
}
