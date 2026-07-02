use std::time::Duration;

use anyhow::{Error, Result};
use tokio::signal::unix::{SignalKind, signal};
use tokio::task::{JoinError, JoinSet};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tracing::Instrument;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, fmt};

#[tokio::main]
async fn main() -> Result<()> {
    setup_trace();

    tracing::info!("Start application");

    let shutdown = CancellationToken::new();

    let mut tasks = JoinSet::new();

    tasks.spawn(
        Service {
            shutdown: shutdown.clone(),
            name: "Service1".into(),
            tick_fail_after: Some(5),
            shutdown_delay: Duration::from_secs(10),
        }
        .serve(),
    );

    tasks.spawn(
        Service {
            shutdown: shutdown.clone(),
            name: "Service2".into(),
            tick_fail_after: None,
            shutdown_delay: Duration::from_secs(10),
        }
        .serve(),
    );

    // Signal
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;

    tokio::select! {
        _ = sigint.recv() => {
            tracing::info!("Received SIGINT");
            shutdown.cancel();
        }

        _ = sigterm.recv() => {
            tracing::info!("Received SIGTERM");
            shutdown.cancel();
        }

        Some(res) = tasks.join_next() => {
            shutdown.cancel();
            handle_result(res);
        }
    }

    while let Some(res) = tasks.join_next().await {
        handle_result(res);
    }

    tracing::info!("Application graceful shutdown");

    Ok(())
}

fn handle_result(result: Result<Result<(), Error>, JoinError>) {
    match result {
        Ok(Ok(())) => tracing::info!("Task exited"),
        Ok(Err(err)) => tracing::error!("{err:#}"),
        Err(join_err) => tracing::error!("Task panicked: {join_err:#}"),
    }
}

struct Service {
    shutdown: CancellationToken,
    name: String,
    tick_fail_after: Option<u8>,
    shutdown_delay: Duration,
}

impl Service {
    pub async fn serve(self) -> Result<()> {
        let Self {
            shutdown,
            name,
            tick_fail_after,
            shutdown_delay,
        } = self;

        let span = tracing::info_span!("", service = name);
        let mut interval = tokio::time::interval(Duration::from_secs(1));

        let mut counter = 0u8;

        async move {
            loop {
                tokio::select! {
                    _ = shutdown.cancelled() => {
                        tracing::info!("shutting down...");

                        sleep(shutdown_delay).await;

                        tracing::info!("graceful shutdown complete");
                        break;
                    }

                    _ = interval.tick() => {
                        tracing::info!("tick");

                        counter += 1;

                        if let Some(limit) = tick_fail_after
                            && counter > limit {
                                tracing::error!("Timeout");
                                return Err(anyhow::anyhow!("Timeout"));
                            }
                    }
                }
            }

            Ok(())
        }
        .instrument(span)
        .await
    }
}

fn setup_trace() {
    let cargo_crate_name = env!("CARGO_CRATE_NAME");
    let base_filter = format!("{cargo_crate_name}=debug");
    let filter = EnvFilter::from(base_filter);

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(std::io::stdout)
                .with_filter(filter),
        )
        .init();
}
