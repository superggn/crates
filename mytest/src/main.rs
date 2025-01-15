use futures::prelude::*;
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{
    fs,
    io::{AsyncBufReadExt, AsyncRead, BufReader, Lines},
};

// extern crate proc_macro;

// use proc_macro;

/// LineStream 内部使用 tokio::io::Lines
#[pin_project]
#[derive(Debug)]
struct LineStream<R> {
    #[pin]
    lines: Lines<BufReader<R>>,
    field2: i32,
    #[pin]
    field3: i32,
}

impl<R: AsyncRead> LineStream<R> {
    /// 从 BufReader 创建一个 LineStream
    pub fn new(reader: BufReader<R>) -> Self {
        Self {
            lines: reader.lines(),
            field2: 2,
            field3: 3,
        }
    }
}

/// 为 LineStream 实现 Stream trait
impl<R: AsyncRead> Stream for LineStream<R> {
    type Item = std::io::Result<String>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        println!("try this: {:?}", self.field2);
        println!("try this3 self: {:?}", self.field3);
        let project = self.project();
        println!("try this2 project: {:?}", project.field2);
        println!("try this3 project: {:?}", project.field3);
        project.lines.poll_next_line(cx).map(Result::transpose)
        // self.project()
        //     .lines
        //     .poll_next_line(cx)
        //     .map(Result::transpose)
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let numbers = vec![1, 2, 3];
    let file = fs::File::open("Cargo.toml").await?;
    let reader = BufReader::new(file);
    let mut st = LineStream::new(reader);
    while let Some(Ok(line)) = st.next().await {
        println!("Got: {}", line);
    }
    Ok(())
}
