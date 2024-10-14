//! Simle TCP server
//! Listens for incoming connections on port 8080 and handles each connection
//! in a separate thread.
//! When a client connects, the server reads data sent by the client, print the
//! request, and responds with a basic HTTP response
//!
//! Start the server `cargo run`
//! Another Terminal run command `echo "Hello" | nc localhost 8080`

use bytes::BytesMut;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    // In Rust, one character is 8bit
    let mut buffer = BytesMut::with_capacity(1);

    // Read data from the stream into a temporary buffer
    let mut temp_buffer = [0; 1];
    let bytes_read = stream.read(&mut temp_buffer)?;

    // Extend the BytesMut buffer with the read data
    buffer.extend_from_slice(&temp_buffer[..bytes_read]);

    let request = String::from_utf8_lossy(&buffer);
    println!("Received request: {}", request);

    // Response
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello from Rust!";

    // Writeh the response back to the client
    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr)?;
    println!("Server listening on {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Handle each connection in a separate thread
                thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Failed to handle client: {:?}", e);
                    }
                });
            }
            Err(e) => eprintln!("Failed to establish a connection: {:?}", e),
        }
    }
    Ok(())
}
