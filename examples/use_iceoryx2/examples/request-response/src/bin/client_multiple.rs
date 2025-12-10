use std::time::{Duration, Instant};

use iceoryx2::prelude::*;
use request_response::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = NodeBuilder::new().create::<ipc::Service>()?;

    let service = node
        .service_builder(&SERVICE.try_into()?)
        .request_response::<Frontend, Backend>()
        .open_or_create()?;

    // Create event service for notifications
    let event_service = node
        .service_builder(&SERVICE_EVENT.try_into()?)
        .event()
        .open_or_create()?;

    let client = service.client_builder().create()?;
    let notifier = event_service.notifier_builder().create()?;
    let listener = event_service.listener_builder().create()?;

    let waitset = WaitSetBuilder::new().create::<ipc::Service>()?;
    let deadline_guard = waitset.attach_deadline(&listener, Duration::from_secs(5))?;

    // Store response times
    let mut response_times = Vec::new();

    // Send 5 requests
    for i in 1..=5 {
        println!("\n=== Request #{} ===", i);

        let start_time = Instant::now();

        // Send request
        let request = client.loan_uninit()?;
        let request = request.write_payload(Frontend::Add);
        let pending_response = request.send()?;

        // Notify server
        notifier.notify()?;

        let on_event = |attachment_id: WaitSetAttachmentId<ipc::Service>| {
            if attachment_id.has_event_from(&deadline_guard) {
                if attachment_id.has_missed_deadline(&deadline_guard) {
                    println!("Response timeout - deadline missed!");
                    return CallbackProgression::Stop;
                }

                // Check for response
                match pending_response.receive() {
                    Ok(Some(backend)) => {
                        let elapsed = start_time.elapsed();
                        println!("✓ Received response: {:?}", *backend);
                        println!("  Response time: {:?}", elapsed);
                        response_times.push((i, elapsed));
                        return CallbackProgression::Stop;
                    }
                    Ok(None) => return CallbackProgression::Continue,
                    Err(e) => {
                        eprintln!("✗ Error receiving response: {:?}", e);
                        return CallbackProgression::Stop;
                    }
                }
            }
            CallbackProgression::Continue
        };

        waitset.wait_and_process(on_event)?;
    }
    // Print summary
    println!("Response Time Summary");

    for (request_num, duration) in &response_times {
        println!("Request #{}: {:>8.2?}", request_num, duration);
    }

    // Calculate statistics
    let times_ms: Vec<f64> = response_times
        .iter()
        .map(|(_, d)| d.as_secs_f64() * 1000.0)
        .collect();

    if !times_ms.is_empty() {
        let avg = times_ms.iter().sum::<f64>() / times_ms.len() as f64;
        let min = times_ms.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = times_ms.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        println!("\nStatistics:");
        println!("  Average: {:.2} ms", avg);
        println!("  Min:     {:.2} ms", min);
        println!("  Max:     {:.2} ms", max);
    }

    Ok(())
}
