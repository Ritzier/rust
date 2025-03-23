mod config;
mod errors;
pub mod whisper;

pub use config::Config;
use errors::Error;

pub type Result<T> = std::result::Result<T, Error>;
