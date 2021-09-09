#![warn(clippy::all)]
#[allow(dead_code)]
use std::str::FromStr;
use async_std::prelude::*;
use async_std::net::{TcpStream, TcpListener};
use futures::stream::StreamExt;

mod http;
//mod threadpool;
#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let port = "8080";
    println!("Listening on {}", port);

    listener.incoming().for_each_concurrent(None, |stream| async move  {
        let stream = stream.unwrap();
        async_std::task::spawn(handle_client(stream));
    }).await;
}

async fn handle_client(mut stream: TcpStream) {

    let mut buffer  = Vec::new();

    stream.read_to_end(&mut buffer).await.expect("Failed to read stream into buffer");

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).await.expect("Failed to send response");

    stream.flush().await.expect("Failed to flush stream response");

    let x = String::from_utf8_lossy(&buffer);
    dbg!(&x);

    let x = http::Request::from_str(&x);

    dbg!(&x);
}

