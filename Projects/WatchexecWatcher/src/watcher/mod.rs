mod event;
#[cfg(test)]
mod tests;
mod watcher_core;

pub use event::Event;
pub use watcher_core::Watcher;
