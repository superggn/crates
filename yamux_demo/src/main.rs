use anyhow::Result;
use futures::{Stream as FutureStream, prelude::*};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{Duration, sleep};
use tokio_util::{
    codec::{Framed, LinesCodec},
    compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt},
};
use tracing::info;
use yamux::{Config, Connection, Mode, Stream};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // server part
    let addr = "127.0.0.1:8001";
    info!("Listening on {:?}", addr);
    tokio::spawn(start_yamux_server(addr));
    sleep(Duration::from_millis(10)).await; // wait server ready
    // client part
    let tcp_stream = TcpStream::connect(addr).await?;
    info!("Connected to server");
    let mut yamux_conn = Connection::new(tcp_stream.compat(), Config::default(), Mode::Client);
    let substream = future::poll_fn(|cx| {
        println!("client side poll_fn polled");
        yamux_conn.poll_new_outbound(cx)
    })
    .await
    .unwrap();
    // prevent blocking (yamux driver)
    let ss = futures::stream::poll_fn(move |cx| yamux_conn.poll_next_inbound(cx));
    tokio::spawn(drive_yamux(ss));
    // msg logic
    let mut framed = Framed::new(substream.compat(), LinesCodec::new());
    let msg = "hello, yamux";
    framed.send(msg).await.unwrap();
    println!("msg put in buffer!");

    let resp = framed.next().await.unwrap().unwrap();

    println!("7");
    println!("resp: {}", resp);
    Ok(())
}

async fn start_yamux_server(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let config = Config::default();
    loop {
        let (tokio_socket, _) = listener.accept().await.unwrap();
        let mut yamux_conn = Connection::new(tokio_socket.compat(), config.clone(), Mode::Server);
        tokio::spawn(async move {
            // ss => stream<item=yamux stream>
            let mut ss = futures::stream::poll_fn(move |cx| yamux_conn.poll_next_inbound(cx));
            let substream = ss.next().await;
            println!("ss: {:?}", ss);
            tokio::spawn(drive_yamux(ss));
            match substream {
                Some(Ok(stream)) => {
                    println!("drop consequent incoming yamux streams");
                    resp_logic(stream).await;
                }
                Some(Err(e)) => {
                    println!("Err: {:?}", e);
                }
                None => {
                    println!("None");
                }
            }
        });
    }
}

/// drive yamux connections by loop calling poll_fn
/// flush buffer
async fn drive_yamux(ss: impl FutureStream<Item = Result<yamux::Stream, yamux::ConnectionError>>) {
    // ss: streams
    // ss.for_each(|s_res| {
    //     drop(s_res);
    //     // future::ready(())
    //     async { () }
    // })
    // .await;
    ss.for_each(|s_res| async move {
        drop(s_res);
        // future::ready(())
    })
    .await
}

async fn resp_logic(stream: yamux::Stream) {
    let mut framed = Framed::new(stream.compat(), LinesCodec::new());

    while let Some(Ok(line)) = framed.next().await {
        println!("Got: {}", line);
        framed
            .send(format!("Hello! I got '{}'", line))
            .await
            .unwrap();
    }
}
