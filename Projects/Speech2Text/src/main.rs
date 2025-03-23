use speech2text::*;

#[tokio::main]
async fn main() -> Result<()> {
    trace::setup_tracing();
    let config = Config::new().await?;

    whisper::run(config)?;

    Ok(())
}
