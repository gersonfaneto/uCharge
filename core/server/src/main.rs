use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};

use http::http::HttpMethod;
use http::middleware::logger::LoggerMiddleware;
use http::request::{ConfirmBody, ConnectBody, Request};
use http::response::Response;
use http::server::{FutureResponse, ServerBuilder};

use models::station::Station;

static mut STATIONS: Vec<Station> = Vec::new();


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    ServerBuilder::new()
        .bind(addr)
        .route("/health", HttpMethod::GET, health_handler)
        .route("/confirm", HttpMethod::POST, confirm_handler)
        .route("/connect", HttpMethod::POST, connect_handler)
        .accept(LoggerMiddleware)
        .build()?
        .run()
        .await?;

    //.route("/login", HttpMethod::POST, handler)
    //.route("/sign_in", HttpMethod::POST, handler)
    //.route("/charge", HttpMethod::POST, handler)
    //.route("/arrived", HttpMethod::POST, handler)
    //.route("/charging", HttpMethod::POST, handler)
    //.route("/charged", HttpMethod::POST, handler)

        //.route("/update", HttpMethod::POST, update_handler)
    Ok(())
}

fn connect_handler(request: Request) -> FutureResponse<'static> {
    match request.body {
        Some(body) => {
            let end = body.find("\0").unwrap();
            let connect_body: Result<ConnectBody, serde_json::Error> =
                serde_json::from_str(&body[..end]);
            match connect_body {
                Ok(body) => {
                    unsafe {
                        STATIONS.push(Station::new(
                            body.position,
                            body.power,
                            body.price,
                            body.name,
                        ));
                    }
                    let response = Response {
                        version: "HTTP/1.1".to_string(),
                        status_code: 200,
                        status_text: "Ok".to_string(),
                        headers: HashMap::new(),
                        body: None,
                    };
                    unsafe {
                        for station in STATIONS.iter() {
                            eprintln!("{}", station);
                        }
                    }
                    eprintln!("[info] - station added");
                    return Box::pin(async move { Ok(response) });
                }
                Err(_) => {
                    get_invalid_body()
                }
            }
        }
        None => {
            get_invalid_body()
        }
    }
}

fn confirm_handler(request: Request) -> FutureResponse<'static> {
    match request.body {
        Some(body) => {
            let end = body.find("\0").unwrap();
            let confirm_body: Result<ConfirmBody, serde_json::Error> =
                serde_json::from_str(&body[..end]);
            match confirm_body {
                Ok(body) => {
                    let html = format!("<html><body><h1>Hello Person {}!</h1></body></html>", body.token);

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
                    return Box::pin(async move { Ok(response) });

            }
                Err(_) => {
                    get_invalid_body()
                }
            }
        }
        None => {
            get_invalid_body()
        }
    }
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

fn get_invalid_body() -> FutureResponse<'static> {
    let response = Response {
        version: "HTTP/1.1".to_string(),
        status_code: 404,
        status_text: "Not Found".to_string(),
        headers: HashMap::new(),
        body: None,
    };
    return Box::pin(async move { Ok(response) });
}
