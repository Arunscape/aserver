#![warn(clippy::all)]
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let listener = TcpListener::bind("0.0.0.0:8080")?;
    let port = "8080";
    println!("Listening on {}", port);

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(stream: TcpStream) {
    dbg!("Hello");
}
