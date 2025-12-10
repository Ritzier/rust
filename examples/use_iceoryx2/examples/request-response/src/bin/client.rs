use std::time::Duration;

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
    let listener = event_service.listener_builder().create()?;
    let notifier = event_service.notifier_builder().create()?;

    // Send request
    let request = client.loan_uninit()?;
    let request = request.write_payload(Frontend::Add);
    let pending_response = request.send()?;

    // Notify server that request is ready
    notifier.notify()?;

    // Create WaitSet with deadline for response timeout
    let waitset = WaitSetBuilder::new().create::<ipc::Service>()?;
    let deadline_guard = waitset.attach_deadline(&listener, Duration::from_secs(5))?;

    let on_event = |attachment_id: WaitSetAttachmentId<ipc::Service>| {
        if attachment_id.has_event_from(&deadline_guard) {
            if attachment_id.has_missed_deadline(&deadline_guard) {
                println!("Response timeout - deadline missed!");
                return CallbackProgression::Stop;
            }

            // Check for response
            match pending_response.receive() {
                Ok(Some(backend)) => {
                    println!("Client received: {:#?}", *backend);
                    return CallbackProgression::Stop;
                }
                Ok(None) => return CallbackProgression::Continue,
                Err(e) => {
                    eprintln!("Failed to receive response: {:?}", e);
                    return CallbackProgression::Stop;
                }
            }
        }
        CallbackProgression::Continue
    };

    waitset.wait_and_process(on_event)?;

    Ok(())
}
