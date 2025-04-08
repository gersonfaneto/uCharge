use std::collections::HashMap;
use std::future::Future;
use std::net::{SocketAddr, TcpStream};
use std::usize;

use http::http::HttpMethod;
use http::middleware::logger::LoggerMiddleware;
use http::request::{ConfirmBody, ConnectBody, LoginBody, Request, UpdateBody};
use http::response::Response;
use http::server::{FutureResponse, ServerBuilder};

use models::driver::Driver;
use models::station::Station;

// replace this for DB
static mut STATIONS: Vec<Station> = Vec::new();
static mut DRIVERS: Vec<Driver> = Vec::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    ServerBuilder::new()
        .bind(addr)
        .route("/health", HttpMethod::GET, health_handler)
        .route("/confirm", HttpMethod::POST, confirm_handler)
        .route("/connect", HttpMethod::POST, connect_handler)
        .route("/update", HttpMethod::POST, update_handler)
        .route("/login", HttpMethod::POST, login_handler)
        .accept(LoggerMiddleware)
        .build()?
        .run()
        .await?;
    //.route("/sign_in", HttpMethod::POST, handler)
    //.route("/charge", HttpMethod::POST, handler)
    //.route("/arrived", HttpMethod::POST, handler)
    //.route("/charging", HttpMethod::POST, handler)
    //.route("/charged", HttpMethod::POST, handler)

    Ok(())
}

fn login_handler(request: Request) -> FutureResponse<'static> {
    match request.body {
        Some(body) => {
            let default = String::from("0");
            let end = request.headers.get("Content-Length").unwrap_or(&default);
            let end: usize = end.parse().unwrap_or(1);
            let login_body: Result<LoginBody, serde_json::Error> =
                serde_json::from_str(&body[..end]);
            match login_body {
                Ok(body) => {
                    let client = Driver::new(body.username, body.password);
                    let mut authentication: bool = false;
                    unsafe {
                        for driver in DRIVERS.iter() {
                            if driver.username == client.username {
                                authentication = driver.password == client.password;
                            }
                        }
                    }
                    if authentication {
                        eprintln!("[info] - client logged");
                    } else {
                        eprintln!("[info] - impossible to login");
                        return get_simple_response(300, String::from("Authetication failed"));
                    }
                    unsafe {
                        for driver in DRIVERS.iter() {
                            if driver.username == client.username {
                                eprint!("LOGGED: ");
                            }
                            eprintln!("{}", driver);
                        }
                    }
                    get_ok_response()
                }
                Err(_) => get_simple_response(400, String::from("Ivalid Body"))
            }
        }
        None => get_simple_response(400, String::from("Request without body"))
    }
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
                        body.old_name,
                    );
                    let mut new_station = Station::new(
                        body.new_position,
                        body.new_power,
                        body.new_price,
                        body.new_name,
                    );
                    let mut index: usize = 0;
                    unsafe {
                        // change for DB access
                        for station in STATIONS.iter() {
                            if station.equals(&old_station) {
                                STATIONS.remove(index);
                                STATIONS.push(new_station);
                                break;
                            }
                            index += 1;
                        }
                    }
                    unsafe {
                        for station in STATIONS.iter() {
                            eprintln!("{}", station);
                        }
                    }
                    get_ok_response()
                }
                Err(_) => get_simple_response(400, String::from("Ivalid Body"))
            }
        }
        None => get_simple_response(400, String::from("Request without body"))
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
                    unsafe {
                        for station in STATIONS.iter() {
                            eprintln!("{}", station);
                        }
                    }
                    get_ok_response()
                }
                Err(_) => get_simple_response(400, String::from("Ivalid Body"))
            }
        }
        None => {
            get_simple_response(400, String::from("Request without body"))
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
                    let html = format!(
                        "<html><body><h1>Hello Person {}!</h1></body></html>",
                        body.token
                    );

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
                    get_simple_response(400, String::from("Ivalid Body"))
                }
            }
        }
        None => {
            get_simple_response(400, String::from("Request without body"))
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

fn get_simple_response(status_code: u16, status_text: String) -> FutureResponse<'static> {
    let response = Response {
        version: "HTTP/1.1".to_string(),
        status_code,
        status_text,
        headers: HashMap::new(),
        body: None,
    };
    Box::pin(async move { Ok(response) })
}

fn get_ok_response() -> FutureResponse<'static> {
    let response = Response {
        version: "HTTP/1.1".to_string(),
        status_code: 200,
        status_text: "Ok".to_string(),
        headers: HashMap::new(),
        body: None,
    };
    Box::pin(async move { Ok(response) })
}
