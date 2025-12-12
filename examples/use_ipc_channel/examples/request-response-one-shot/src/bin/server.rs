use std::fs;

use ipc_channel::ipc::IpcOneShotServer;
use request_response::{Request, Response};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Server: Starting...");

    // Create one-shot server for initial bootstrap
    let (server, server_name): (IpcOneShotServer<Request>, String) = IpcOneShotServer::new()?;

    println!("SERVER_NAME={}", server_name);

    // Write `SERVER_NAME` to `.env` file
    fs::write(
        format!("{}/.env", env!("CARGO_MANIFEST_DIR")),
        format!("SERVER_NAME={server_name}"),
    )?;

    // Accept first connection and get initial request
    let (rx, first_request) = server.accept()?;
    println!("Server: Received first request");

    // Process and respond to first request
    let response = Response {
        message: "Response #1".to_string(),
    };
    first_request
        .response_sender
        .send(response)
        .expect("Failed to send response");

    // Handle subsequent requests from the established channel
    println!("Server: Waiting for more requests...");
    let mut request_count = 2;

    loop {
        match rx.recv() {
            Ok(request) => {
                println!("Server: Received request #{}", request_count);

                let response = Response {
                    message: format!("Response #{}", request_count),
                };

                if let Err(e) = request.response_sender.send(response) {
                    eprintln!("Server: Failed to send response: {}", e);
                }

                request_count += 1;
            }
            Err(e) => {
                println!("Server: Client disconnected or error: {}", e);
                break;
            }
        }
    }

    println!("Server: Shutting down");
    Ok(())
}
