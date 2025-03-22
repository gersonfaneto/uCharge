#[derive(Debug)]
pub enum Host {
    Vehicle,
    Station,
}

#[derive(Debug)]
pub struct Request<T> {
    host: Host,
    data: T,
}

impl<T> Request<T> {
    pub fn new(host: Host, data: T) -> Self {
        Self { host, data }
    }
}
