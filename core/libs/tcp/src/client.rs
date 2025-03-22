use std::io::Write;
use std::net::TcpStream;

use serde::Serialize;

pub fn connect(addr: &str) -> TcpStream {
    TcpStream::connect(addr).unwrap()
}

pub fn send<T>(mut stream: TcpStream, data: T)
where
    T: Serialize,
{
    let data_ser = serde_json::to_string(&data).unwrap();
    let data_bytes = data_ser.into_bytes();
    stream.write_all(&data_bytes).unwrap();
}
