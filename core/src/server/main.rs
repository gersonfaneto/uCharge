use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{result, str, thread};

type Result<T> = result::Result<T, ()>;

const PORT: &str = "3333";

fn main() -> Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{PORT}")).map_err(|err| {
        eprintln!("[ERROR]: Couldn't bind to port '{PORT}' : {err}");
    })?;

    println!("[INFO]: Listening on port {PORT}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!(
                    "[INFO]: Connection established with {}",
                    stream.peer_addr().unwrap()
                );
                thread::spawn(|| {
                    let _ = handle_client(stream);
                });
            }
            Err(err) => {
                eprintln!("[ERROR]: Failed to connect to : {err}");
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0 as u8; 4096];

    loop {
        let stream_addr = stream.peer_addr().unwrap();

        let size = stream.read(&mut buffer).map_err(|err| {
            eprintln!("[ERROR]: Failed to read from connection {stream_addr} : {err}",);
        })?;

        if size > 0 {
            stream.write(&buffer[0..size]).unwrap();
            println!(
                "[INFO]: Received '{data}' from {stream_addr}",
                data = std::str::from_utf8(&buffer[0..size - 1]).unwrap()
            );
        } else {
            println!("[INFO]: Connection with {stream_addr} closed");
            break;
        }
    }

    Ok(())
}
