use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv::{CommandRequest, CommandResponse, MemTable, Service, ServiceInner, SledDb};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // let service: Service = Service::new(MemTable::new());
    // let service: Service<SledDb> = Service::new(SledDb::new(""));
    // let path = tempfile::tempdir("/tmp/kvserver");
    let temp_dir = tempfile::tempdir()?;
    let service: Service<SledDb> = ServiceInner::new(SledDb::new(temp_dir))
        .fn_before_send(|res| match res.message.as_ref() {
            "" => res.message = "altered. Original message is empty.".into(),
            s => res.message = format!("altered: {}", s),
        })
        .into();
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);
        let svc = service.clone();
        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();
            while let Some(Ok(cmd)) = stream.next().await {
                let res = svc.execute(cmd);
                stream.send(res).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
