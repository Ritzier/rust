use thiserror::Error;

use crate::app::Event;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Crossterm error")]
    Crossterm,
    #[error("")]
    EventSend(#[from] tokio::sync::mpsc::error::SendError<Event>),
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),
    #[error("Unexpected: {0}")]
    Unexpected(String),
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
}
