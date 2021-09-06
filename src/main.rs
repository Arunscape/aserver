#![warn(clippy::all)]
use std::net::{TcpListener, TcpStream};
use std::thread;

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

fn handle_client(stream: TcpStream) {
    dbg!("Hello");
}
