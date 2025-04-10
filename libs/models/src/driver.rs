use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Driver {
    pub username: String,
    pub password: String
}

impl Driver {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password
        }
    }
}


impl std::fmt::Display for Driver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "username {}, password {}",
            self.username, self.password,
        )
    }
}

