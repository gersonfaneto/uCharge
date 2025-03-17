use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::result;
use std::{fmt, thread};

const PORT: u16 = 3333;
const SAFE_MODE: bool = true;

type Result<T> = result::Result<T, ()>;

struct Sensible<T>(T);

impl<T: fmt::Display> fmt::Display for Sensible<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;

        if SAFE_MODE {
            "[REDACTED]".fmt(f)
        } else {
            inner.fmt(f)
        }
    }
}

fn main() -> Result<()> {
    let address = format!("0.0.0.0:{PORT}");

    let listener = TcpListener::bind(address.parse::<String>().unwrap()).map_err(|err| {
        eprintln!(
            "ERROR: could not bind {address}: {err}",
            address = Sensible(&address),
            err = Sensible(err)
        )
    })?;

    println!(
        "[INFO]: Listening to {address}",
        address = Sensible(address)
    );

    for stream in listener.incoming() {
        match stream {
            Err(err) => {
                eprintln!("[ERROR]: Couldn't accept connection: {err}");
            }
            Ok(stream) => {
                let addr = Sensible(stream.local_addr().unwrap().ip());

                println!("[INFO]: Received connection from {addr}");

                thread::spawn(|| handle_stream(stream));
            }
        }
    }

    Ok(())
}

fn handle_stream(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 8192];

    loop {
        let addr = Sensible(stream.local_addr().unwrap().ip());

        let bytes = stream.read(&mut buffer).map_err(|err| {
            eprintln!("[ERROR]: Failed to read data from connection: {err}");
        })?;

        if bytes == 0 {
            println!("[INFO]: Closed connection from {addr}");
            return Ok(());
        }

        match String::from_utf8(buffer.to_vec()) {
            Ok(str) => print!("[INFO]: Message from {}: {}", addr, str),
            Err(err) => eprintln!("[ERROR]: Failed to convert data: {err}"),
        }

        stream.write(&buffer).map_err(|err| {
            eprintln!("[ERROR]: Failed to write data to connection: {err}");
        })?;

        stream.flush().map_err(|err| {
            eprintln!("[ERROR]: Failed to flush connection: {err}");
        })?;
    }
}
