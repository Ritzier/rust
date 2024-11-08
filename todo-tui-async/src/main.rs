use app::App;

mod app;
mod error;

type Result<T> = std::result::Result<T, error::Error>;

#[tokio::main]
async fn main() -> Result<()> {
    App::new("todo.json".to_string(), 60f64).await?.run().await
}
