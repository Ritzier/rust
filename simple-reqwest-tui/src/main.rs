mod app;
mod error;

use app::App;
use error::Error;

type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    App::new(60f64, 10f64)?.run().await
}
