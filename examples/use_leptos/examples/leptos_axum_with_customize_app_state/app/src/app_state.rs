use std::sync::Arc;
use std::sync::atomic::AtomicU8;

use axum::extract::FromRef;
use leptos::config::LeptosOptions;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub number: Arc<AtomicU8>,
    pub leptos_options: LeptosOptions,
}
