use std::time::Duration;

use iceoryx2::prelude::*;
use request_response::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = NodeBuilder::new().create::<ipc::Service>()?;

    // Create request-response service
    let service = node
        .service_builder(&SERVICE.try_into()?)
        .request_response::<Frontend, Backend>()
        .open_or_create()?;

    // Create event service for notifications
    let event_service = node
        .service_builder(&SERVICE_EVENT.try_into()?)
        .event()
        .open_or_create()?;

    let server = service.server_builder().create()?;
    let listener = event_service.listener_builder().create()?;

    // Create WaitSet and attach the listener
    let waitset = WaitSetBuilder::new().create::<ipc::Service>()?;
    let listener_guard = waitset.attach_notification(&listener)?;

    // Optional: add an interval for periodic checks
    let interval_guard = waitset.attach_interval(Duration::from_secs(1))?;

    let on_event = |attachment_id: WaitSetAttachmentId<ipc::Service>| {
        // Handle incoming requests when notified
        if attachment_id.has_event_from(&listener_guard) {
            while let Ok(Some(request)) = server.receive() {
                println!("Server received: {:#?}", *request);

                if let Ok(response) = request.loan_uninit() {
                    let response = response.write_payload(Backend::Data(10));
                    let _ = response.send();
                }
            }
        }

        // Handle interval wake-ups (health check, logging, etc.)
        if attachment_id.has_event_from(&interval_guard) {
            println!("Server heartbeat...");
        }

        CallbackProgression::Continue
    };

    waitset.wait_and_process(on_event)?;

    Ok(())
}
