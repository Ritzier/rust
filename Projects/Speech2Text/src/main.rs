use speech2text::*;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new().await?;

    whisper::run(config)?;

    Ok(())
}
