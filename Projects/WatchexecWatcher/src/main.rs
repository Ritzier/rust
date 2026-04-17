use watchexec_watcher::Watcher;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Watcher {
        mut watchexec_task,
        mut event_receiver,
        startup_rx,
        mut include_updater_task,
        include_sender,
    } = Watcher::build("Cargo.toml")?;

    // Wait for ready
    startup_rx.await?;

    // Add `watch` path
    include_sender.send(vec!["**/*.rs".into()]).await??;

    loop {
        tokio::select! {
            Some(event) = event_receiver.recv() => {
                println!("Get: {event:#?}");
            }

            res = &mut watchexec_task => {
                match res {
                    Ok(inner) => {
                        if let Err(e) = inner {
                            eprintln!("Watchexec error: {e}");
                        }
                    }
                    Err(e) => eprintln!("Join error: {e}"),
                }
                break;
            }

            res = &mut include_updater_task => {
                match res {
                    Ok(inner) => match inner  {
                        Ok(()) => {
                            eprintln!("IncludeUpdater closed")
                        }
                        Err(e) => {
                            eprintln!("IncldueUpdater error: {e}")
                        }
                    }
                    Err(e) => {
                        eprintln!("Join error: {e}")
                    }
                }
            }
        }
    }

    Ok(())
}
