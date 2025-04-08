use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::usize;

use http::http::HttpMethod;
use http::middleware::logger::LoggerMiddleware;
use http::request::{ConfirmBody, ConnectBody, Request, UpdateBody};
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
        .route("/update", HttpMethod::POST, update_handler)
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

    Ok(())
}

fn update_handler(request: Request) -> FutureResponse<'static> {
    match request.body {
        Some(body) => {
            let default = String::from("0");
            let end = request.headers.get("Content-Length").unwrap_or(&default);
            let end: usize = end.parse().unwrap_or(1);
            let update_body: Result<UpdateBody, serde_json::Error> =
                serde_json::from_str(&body[..end]);
            match update_body {
                Ok(body) => {
                    let mut old_station = Station::new(
                        body.old_position,
                        body.old_power,
                        body.old_price,
                        body.old_name
                    );
                    let mut new_station = Station::new(
                        body.new_position,
                        body.new_power,
                        body.new_price,
                        body.new_name
                    );
                   let mut index:usize = 0; 
                   unsafe { // change for DB access
                        for station in STATIONS.iter() {
                            if station.equals(&old_station) {
                                STATIONS.remove(index);                        
                                STATIONS.push(new_station);
                                break;
                            }
                            index += 1;
                        }
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

fn connect_handler(request: Request) -> FutureResponse<'static> {
    match request.body {
        Some(body) => {
            let default = String::from("0");
            let end = request.headers.get("Content-Length").unwrap_or(&default);
            let end: usize = end.parse().unwrap_or(1);
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
                    return Box::pin(async move { Ok(response) });
                }
                Err(_) => {
                    get_invalid_body()
                }
            }
        }
        None => {
                    eprintln!("[error] not have bbody");
            get_invalid_body()
        }
    }
}

fn confirm_handler(request: Request) -> FutureResponse<'static> {
    match request.body {
        Some(body) => {
            let default = String::from("0");
            let end = request.headers.get("Content-Length").unwrap_or(&default);
            let end: usize = end.parse().unwrap_or(1);
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
                Err(e) => {
                    eprintln!("{e}");
                    get_invalid_body()
                }
            }
        }
        None => {
                    eprintln!("invali body!!!");
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
        status_code: 401,
        status_text: "Invalid body".to_string(),
        headers: HashMap::new(),
        body: None,
    };
    return Box::pin(async move { Ok(response) });
}
