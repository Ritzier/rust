use std::path::PathBuf;

use thiserror::Error;
use watchexec::error::CriticalError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration path not exists: {path}")]
    ConfigurationNotExists { path: PathBuf },

    #[error("watchexec critical error")]
    WxCritical(#[from] Box<CriticalError>),

    #[error("path is not valid UTF-8: {pathbuf}")]
    PathIsNotValidUTF8 { pathbuf: PathBuf },
}
