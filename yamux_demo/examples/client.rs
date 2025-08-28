use futures::sink::SinkExt;
use futures::stream::StreamExt;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use yamux::{Config, Mode};
use yamux_demo::handle::spawn_yamux_driver;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080";
    let tcp_stream = TcpStream::connect(addr).await?;
    let yamux_handle = spawn_yamux_driver(tcp_stream, Mode::Client, Config::default());
    let yamux_stream = yamux_handle.open_outbound().await.unwrap();
    let mut framed = Framed::new(yamux_stream.compat(), LinesCodec::new());
    println!("frame init done!");
    let msg = "Hello, this is Tyr!".to_string();
    let send_res = framed.send(msg).await;
    println!("send_res: {:?}", send_res);
    let response = framed.next().await;
    println!("response: {:?}", response);

    Ok(())
}
