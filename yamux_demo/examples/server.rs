use futures::sink::SinkExt;
use futures::stream::StreamExt;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use yamux::{Config, Mode};
use yamux_demo::handle::spawn_yamux_driver;

#[tokio::main]
async fn main() {
    start_server().await;
    tokio::time::sleep(Duration::from_secs(50)).await;
}

async fn start_server() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("listener: {:?}", listener);
    tokio::spawn(async move {
        loop {
            println!("start loop!");
            let (tcp_stream, _) = listener.accept().await.unwrap();
            println!("tcp_stream: {:?}", tcp_stream);
            let mut yamux_handle = spawn_yamux_driver(tcp_stream, Mode::Server, Config::default());
            println!("yamux_handle ready!");
            let stream_res = yamux_handle.next_incoming().await;
            println!("stream_res done!");
            match stream_res {
                Some(stream) => {
                    let mut framed = Framed::new(stream.compat(), LinesCodec::new());
                    while let Some(Ok(line)) = framed.next().await {
                        let resp = format!("resp_{line}");
                        println!("resp: {:?}", resp);
                        let _ = framed.send(resp).await;
                    }
                }
                None => (),
            }
        }
    });
}
