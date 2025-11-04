pub mod app;
pub mod pages;

#[cfg(feature = "ssr")]
pub mod ssr;

#[cfg(not(feature = "ssr"))]
pub mod hljs;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    use tracing_subscriber::fmt;
    use tracing_subscriber_wasm::MakeConsoleWriter;

    fmt()
        .with_writer(MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG))
        .with_ansi(false)
        .without_time()
        .init();
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_lazy(App)
}
