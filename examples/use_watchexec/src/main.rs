use use_watchexec::Watcher;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Watcher {
        mut watchexec_task,
        mut event_receiver,
        startup_rx,
    } = Watcher::build(["Cargo.toml"])?;

    startup_rx.await?;

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
        }
    }

    Ok(())
}
