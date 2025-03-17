use std::io::{Read, Write};
use std::net::TcpStream;
use std::result;
use std::str::from_utf8;

type Result<T> = result::Result<T, ()>;

const PORT: &str = "3333";

fn main() -> Result<()> {
    let mut stream = TcpStream::connect(format!("0.0.0.0:{PORT}")).map_err(|err| {
        eprintln!(
            "[ERROR]: Couldn't connect to {addr}: {err}",
            addr = format!("0.0.0.0:{PORT}")
        );
    })?;

    println!(
        "[INFO]: Connection stablished with {addr}",
        addr = format!("0.0.0.0:{PORT}")
    );

    let msg = b"Hello, World!";

    stream.write(msg).unwrap();

    println!("[INFO]: Message sent, awaiting reply...");

    let mut buffer = [0 as u8; 13];

    stream.read_exact(&mut buffer).map_err(|err| {
        eprintln!("[ERROR]: Connection failed to reply: {err}");
    })?;

    if &buffer == msg {
        println!("[INFO]: Connection replied correctly");
    } else {
        let text = from_utf8(&buffer).unwrap();
        println!("[INFO]: Connection replied unexpectedly: {}", text);
    }

    Ok(())
}
