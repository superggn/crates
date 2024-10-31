use mytest::{get, post, Opts, SubCommand};

use anyhow::Result;
use clap::Parser;
use reqwest::header;

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut headers = header::HeaderMap::new();
    // add default rust header (随便改的)
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let result = match opts.sub_cmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}
