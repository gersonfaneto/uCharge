use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {}

impl Owner {
    pub fn new() -> Self {
        Self {}
    }
}
