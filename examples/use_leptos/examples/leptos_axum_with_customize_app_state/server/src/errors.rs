use std::net::SocketAddr;

use leptos::config::errors::LeptosConfigError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("LeptosConfig: {0}")]
    LeptosConfig(#[from] LeptosConfigError),

    #[error("{addr} Adress is used: {source}")]
    AdressUsed {
        addr: SocketAddr,
        #[source]
        source: std::io::Error,
    },

    #[error("Io: {0}")]
    Io(#[from] std::io::Error),
}
