mod audio;
mod config;
mod errors;
mod speech2text;
pub mod trace;
mod whisper;

use config::Config;
pub use speech2text::Speech2Text;

pub use errors::Error;
pub type Result<T> = std::result::Result<T, Error>;
