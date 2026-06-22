use std::sync::Arc;
use std::sync::atomic::AtomicU8;

#[derive(Clone)]
pub struct AppState {
    pub number: Arc<AtomicU8>,
}
