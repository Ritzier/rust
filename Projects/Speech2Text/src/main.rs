use speech2text::*;

#[tokio::main]
async fn main() -> Result<()> {
    trace::setup_tracing();

    let speech2text = Speech2Text::new().await?;
    speech2text.run().await?;

    Ok(())
}
