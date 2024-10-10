/// Using `ncat` for listen on 6142 port
/// `ncat -l 6142`
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    println!("Created stream");

    let result = stream.write_all(b"Hello, Tokio\n").await;
    println!("Wrote to stream; sucess={:?}", result.is_ok());

    Ok(())
}
