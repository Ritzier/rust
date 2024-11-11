use fantoccini::{ClientBuilder, Locator};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("CmdError: {0}")]
    Cmd(#[from] fantoccini::error::CmdError),
    #[error("NewSessionError: {0}")]
    NewSession(#[from] fantoccini::error::NewSessionError),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connecting using "native" TLS
    let client = ClientBuilder::native()
        // webdriver default port: 4444
        .connect("http://localhost:4444")
        .await?;

    // go to Wikipedia page for Foobar
    client.goto("https://en.wikipedia.org/wiki/Foobar").await?;
    let url = client.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");

    // click "Foo (disambiguation)"
    client
        .find(Locator::Css(".mw-disambig"))
        .await?
        .click()
        .await?;

    // click "Foo Lake"
    client
        .find(Locator::LinkText("Foo Lake"))
        .await?
        .click()
        .await?;

    let url = client.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

    client.close().await?;

    Ok(())
}
