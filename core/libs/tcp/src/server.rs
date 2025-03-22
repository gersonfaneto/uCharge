use std::io::BufRead;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::models::request::Request;

pub fn listen(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_client(stream);
        });
    }
}

fn handle_client(client: TcpStream) {
    let buf_reader = BufReader::new(&client);
    let request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("{:?}", request);
    let request: &str = request[0].as_str();
    let request: Request<String> = serde_json::from_str(request).expect("REASON");
    println!("{:?}", request);
}
