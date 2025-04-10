use std::future::Future;
use std::pin::Pin;

use crate::request::Request;
use crate::response::Response;
use crate::server::FutureResponse;

pub type FutureRequest<'a> = Pin<
    Box<dyn Future<Output = Result<Request, Box<dyn std::error::Error + Send + 'a>>> + Send + 'a>,
>;

pub trait Middleware: Send + Sync {
    fn on_request<'a>(&self, request: Request) -> FutureRequest<'a>;
    fn on_response<'a>(&self, response: Response) -> FutureResponse<'a>;
}
