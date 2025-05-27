use futures::prelude::*;
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};
use yamux::{Config, Connection, Mode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a server (or use TcpListener for the server side)
    let socket = TcpStream::connect("127.0.0.1:12345").await?;

    // Create a Yamux connection in client mode
    let config = Config::default();
    let mut yamux = Connection::new(socket.compat(), config, Mode::Client);

    // Open a new outbound Yamux stream
    let mut outbound = futures::future::poll_fn(|cx| yamux.poll_new_outbound(cx)).await?;

    // Write some data
    outbound.write_all(b"hello yamux!").await?;

    // Read a response
    let mut buf = [0u8; 1024];
    let n = outbound.read(&mut buf).await?;
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}
