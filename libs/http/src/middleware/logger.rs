use crate::middleware::middleware::FutureRequest;
use crate::middleware::middleware::Middleware;
use crate::request::Request;
use crate::response::Response;
use crate::server::FutureResponse;

#[derive(Debug, Clone)]
pub struct LoggerMiddleware;

impl Middleware for LoggerMiddleware {
    fn on_request<'a>(&self, request: Request) -> FutureRequest<'a> {
        println!("[INFO]: {:#?}", request);

        Box::pin(async move { Ok(request) })
    }

    fn on_response<'a>(&self, response: Response) -> FutureResponse<'a> {
        println!("[INFO]: {:#?}", response);
        Box::pin(async move { Ok(response) })
    }
}
