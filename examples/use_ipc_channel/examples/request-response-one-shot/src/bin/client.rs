use dotenvy::dotenv;
use ipc_channel::ipc::{self, IpcSender};
use request_response::*;
use std::env;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Client: Starting...");

    // Get server name from environment variable
    dotenv()?;
    let server_name = env::var("SERVER_NAME")?;

    println!("Client: Connecting to server: {}", server_name);

    // Connect to server
    let connect_start = Instant::now();
    let server_tx: IpcSender<Request> = IpcSender::connect(server_name)?;
    let connect_time = connect_start.elapsed();

    println!("Client: Connected successfully (took {:?})", connect_time);

    // Collect response times
    let mut response_times = Vec::new();

    // Send multiple requests
    for i in 1..=5 {
        let (response_tx, response_rx) = ipc::channel()?;

        let request = Request {
            response_sender: response_tx,
        };

        // Start timing before sending request
        let start = Instant::now();
        server_tx.send(request).expect("Failed to send request");

        // Wait for response
        match response_rx.recv() {
            Ok(response) => {
                let elapsed = start.elapsed();
                response_times.push(elapsed);

                println!(
                    "Client: Received response #{}: '{}' (took {:?})",
                    i, response.message, elapsed
                );
            }
            Err(e) => {
                eprintln!("Client: Failed to receive response: {}", e);
            }
        }

        thread::sleep(Duration::from_millis(500));
    }

    // Calculate and display statistics
    println!("\n=== Performance Statistics ===");
    println!(
        "Connection time: {:?} ({:.2} ms)",
        connect_time,
        connect_time.as_secs_f64() * 1000.0
    );

    if !response_times.is_empty() {
        let total: Duration = response_times.iter().sum();
        let avg = total / response_times.len() as u32;
        let min = response_times.iter().min().unwrap();
        let max = response_times.iter().max().unwrap();

        println!("\n=== Response Time Statistics ===");
        println!("Total requests: {}", response_times.len());
        println!("Average: {:?}", avg);
        println!("Min: {:?}", min);
        println!("Max: {:?}", max);
        println!("Total: {:?}", total);

        println!("\nIndividual times:");
        for (i, time) in response_times.iter().enumerate() {
            println!(
                "  Request #{}: {:?} ({:.2} ms)",
                i + 1,
                time,
                time.as_secs_f64() * 1000.0
            );
        }
    }

    println!("\nClient: Done");

    Ok(())
}
