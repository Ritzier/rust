use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub struct Trace;

impl Trace {
    pub fn setup() {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    if cfg!(debug_assertions) {
                        format!("{}=debug,app=debug", env!("CARGO_CRATE_NAME")).into()
                    } else {
                        format!("{}=info,app=info", env!("CARGO_CRATE_NAME")).into()
                    }
                }),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}
