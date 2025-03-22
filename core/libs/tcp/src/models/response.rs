pub struct Response<T> {
    data: T,
}

impl<T> Response<T> {
    fn new(data: T) -> Self {
        Self { data }
    }
}
