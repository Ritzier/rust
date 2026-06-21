#[cfg(feature = "ssr")]
#[cfg(debug_assertions)]
mod debug;
#[cfg(feature = "ssr")]
mod errors;
#[cfg(feature = "ssr")]
mod server;

#[cfg(feature = "ssr")]
#[cfg(debug_assertions)]
pub use debug::Env;
#[cfg(feature = "ssr")]
pub use errors::Error;
#[cfg(feature = "ssr")]
pub use server::Server;
