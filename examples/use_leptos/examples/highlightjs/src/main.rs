#[tokio::main]
async fn main() {
    use highlightjs::ssr::*;

    Trace::setup();
    Server::setup().await;
}
