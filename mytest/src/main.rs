// use anyhow::Result;
// use futures::prelude::*;
// use tokio::{fs::File, io::AsyncWriteExt};

// #[tokio::main]
// async fn main() -> Result<()> {
//     let file_sink = writer(File::create("/tmp/hello").await?);
//     // pin_mut 可以把变量 pin 住
//     futures::pin_mut!(file_sink);
//     if let Err(_) = file_sink.send("hello\\n").await {
//         println!("Error on send");
//     }
//     if let Err(_) = file_sink.send("world!\\n").await {
//         println!("Error on send");
//     }
//     Ok(())
// }

// /// 使用 unfold 生成一个 Sink 数据结构
// fn writer<'a>(file: File) -> impl Sink<&'a str> {
//     sink::unfold(file, |mut file, line: &'a str| async move {
//         file.write_all(line.as_bytes()).await?;
//         eprint!("Received: {}", line);
//         Ok::<_, std::io::Error>(file)
//     })
// }
use anyhow::Result;
use futures::sink::{self, Sink, SinkExt};
use futures::stream::Stream;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use tokio::io::AsyncBufRead;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};
// use tokio::net::TcpStream;
// use std::net::{TcpStream};
use serde::{Deserialize, Serialize};
// use std::fs::File;
use tokio::net::TcpStream;
use tokio::{self, fs::File, io::AsyncWriteExt};

#[derive(Deserialize, Serialize)]
struct MyStruct {
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let file_sink = writer(File::create("/tmp/hello").await?);
    futures::pin_mut!(file_sink);
    if let Err(_) = file_sink.send("hello\n").await {
        println!("error on send!");
    }
    if let Err(_) = file_sink.send("world\n").await {
        println!("Error on send");
    }
    // todo!();
    Ok(())
}

fn writer<'a>(file: File) -> impl Sink<&'a str> {
    sink::unfold(file, |mut file, line: &'a str| async move {
        file.write_all(line.as_bytes()).await?;
        eprint!("Hahaha IT'S MY LIFE CS GOGOGO!!! Received: {}", line);
        Ok::<_, std::io::Error>(file)
    })
}
