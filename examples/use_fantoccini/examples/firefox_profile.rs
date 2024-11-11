use fantoccini::ClientBuilder;
use serde_json::{Map, Value};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("CmdError: {0}")]
    Cmd(#[from] fantoccini::error::CmdError),
    #[error("NewSessionError: {0}")]
    NewSession(#[from] fantoccini::error::NewSessionError),
    #[error("Json: {0}")]
    Json(#[from] serde_json::Error),
}
/// https://developer.mozilla.org/en-US/docs/Web/WebDriver/Capabilities

/// INFO: When your profile is opening, Fantoccini would return error:
/// Error: NewSession(SessionNotCreated(WebDriver { error: Session NotCreated,
/// message: "Failed to set preferences: unknown error", stacktrace: "", data:
/// None }))

/// INFO: When Fantoccini didn't close the client, the error would be displayed:
/// Error: NewSession(SessionNotCreated(WebDriver { error: SessionNotCreated,
/// message: "Session is already started", stacktrace: "", data: None }))
/// Just close the geckordiver and rerun it
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create a new map for capabilities
    let mut capabilities = Map::new();

    // Firefox Options
    let mut firefox_options = Map::new();
    firefox_options.insert(
        "args".to_string(),
        serde_json::to_value(vec![
            "--profile",
            "/home/ritzier/.mozilla/firefox/0rjf97hh.default",
        ])?,
    );

    // Insert the Firefox options into capabilities
    capabilities.insert(
        "moz:firefoxOptions".to_string(),
        Value::Object(firefox_options),
    );

    // Connecting using "native" TLS
    let client = ClientBuilder::native()
        // build with capabilities
        .capabilities(capabilities)
        // webdriver default port: 4444
        .connect("http://localhost:4444")
        .await?;

    // go to Github page
    client.goto("https://github.com").await?;

    // Close the client
    client.close().await?;

    Ok(())
}
