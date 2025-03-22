use tcp::client::{connect, send};
use tcp::models::request::{Host, Request};

fn main() {
    let stream = connect("0.0.0.0:7878");
    let req = Request::new(Host::Vehicle, "Hello from the Client!");
    send(stream, req);
}
