use std::collections::HashMap;
use std::net::SocketAddr;

use http::http::HttpMethod;
use http::middleware::logger::LoggerMiddleware;
use http::request::Request;
use http::response::Response;
use http::server::{FutureResponse, ServerBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));

    let _ = ServerBuilder::new()
        .bind(addr)
        .route("/health", HttpMethod::GET, health_handler)
        .accept(LoggerMiddleware)
        .build()?
        .run()
        .await?;

    Ok(())
}

fn health_handler(_: Request) -> FutureResponse<'static> {
    let html = "<html><body><h1>Hello, Rustacean!</h1></body></html>";

    let response = Response {
        version: "HTTP/1.1".to_string(),
        status_code: 200,
        status_text: "OK".to_string(),
        headers: {
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "text/html".to_string());
            headers
        },
        body: Some(html.to_string()),
    };

    Box::pin(async move { Ok(response) })
}
