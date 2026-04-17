use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum IncludeError {
    #[error("path not exists: {pathbuf}")]
    PathNotExists { pathbuf: PathBuf },

    #[error("path resolution failed")]
    Absolute(#[source] std::io::Error),

    #[error("invalid glob pattern")]
    Glob(#[from] globset::Error),

    #[error("path is not valid UTF-8: {pathbuf}")]
    PathIsNotValidUTF8 { pathbuf: PathBuf },
}
