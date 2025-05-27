use futures::prelude::*;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::compat::{Compat, FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};
use yamux::{Config, Connection, Mode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a server (or use TcpListener for the server side)
    let socket = TcpStream::connect("127.0.0.1:8000").await?;
    println!("start listening on 8000!");
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    while let Ok((raw_stream, addr)) = listener.accept().await {
        let mut server_conn = Connection::new(raw_stream.compat(), Config::default(), Mode::Server);

        tokio::spawn(async move {
            while let Some(stream) = server_conn.poll_next_inbound(&mut cx).await? {
                // 3. 为每个流启动处理逻辑
                tokio::spawn(async move {
                    handle_stream(yamux_stream).await;
                });
            }
        });
    }

    // Create a Yamux connection in client mode
    let config = Config::default();
    let mut yamux = Connection::new(socket.compat(), config, Mode::Server);

    // Open a new outbound Yamux stream
    let mut inbound = futures::future::poll_fn(|cx| yamux.poll_next_inbound(cx))
        .await
        .unwrap()?;

    // Write some data
    // Read a response
    let mut buf = [0u8; 1024];
    let n = inbound.read(&mut buf).await?;
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
    inbound.write_all(b"hello yamux!").await?;

    println!("ha!");
    Ok(())
}
