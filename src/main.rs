#![warn(clippy::all)]
#[allow(dead_code)]
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::str::FromStr;

mod http;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    let port = "8080";
    println!("Listening on {}", port);

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            handle_client(stream);
        });
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {

    let mut buffer  = Vec::new();

    stream.read_to_end(&mut buffer).expect("Failed to read stream into buffer");

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).expect("Failed to send response");

    stream.flush().expect("Failed to flush stream response");

    let x = String::from_utf8_lossy(&buffer);

    let x = http::Request::from_str(&x);
}

