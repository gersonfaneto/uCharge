use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    data: T,
}

impl<T> Response<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}
