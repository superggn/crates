use futures::stream::{self, StreamExt};
// use std::collections::Iterator;
use tokio;

#[tokio::main]

async fn main() {
    let my_stream = stream::iter(vec![1, 2, 3, 4]);
    let job = my_stream.for_each(|item| async move {
        println!("Got item: {}", item);
    });
    println!("haha");
    std::thread::sleep(std::time::Duration::from_secs(1));
    // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("sleep over");
    job.await;
}

struct AA {}

use std::iter::Iterator;

impl Iterator for AA {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        Some(1)
    }
}
