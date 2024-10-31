use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use reqwest::Url;
use std::str::FromStr;

// 定义 HTTPie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

/// 我用 rust 实现的 httpie 平替
#[derive(Parser, Debug)]
#[command(version = "1.0", author = "somebody <somebody@gmail.com>")]
pub struct Opts {
    #[clap(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Parser, Debug)]
pub struct Get {
    #[arg(value_parser = parse_url)]
    pub url: String,
}

#[derive(Parser, Debug)]
pub struct Post {
    #[arg(value_parser = parse_url)]
    pub url: String,
    #[arg(value_parser = parse_kv_pair)]
    pub body: Vec<KvPair>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct KvPair {
    pub k: String,
    pub v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<KvPair> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

/// check if url is legal
fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}
