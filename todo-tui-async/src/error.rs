use thiserror::Error;

use crate::app;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),
    #[error("Serde: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Send error: {0}")]
    SendError(#[from] tokio::sync::mpsc::error::SendError<app::Event>),
    #[error("Crossterm error")]
    CrosstermError,
}
