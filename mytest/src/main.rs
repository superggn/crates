use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("File not found: {0}")]
    NotFound(String),

    #[error("Permission denied")]
    PermissionDenied,
}

fn read_file(path: &str) -> Result<String, MyError> {
    if path == "not_found.txt" {
        Err(MyError::NotFound(path.to_string()))
    } else if path == "denied.txt" {
        Err(MyError::PermissionDenied)
    } else {
        Ok("File content".to_string())
    }
}

fn run(s: &str) -> Result<()> {
    let content = read_file("not_found.txt")
        .context(format!("anyhowhowhow~ Failed to read the file: {}", s))?; // 使用 anyhow 添加上下文
    println!("content: {}", content);
    Ok(())
}

fn main() {
    if let Err(e) = run("haha str") {
        eprintln!("Application error: {:?}", e);
        println!("====================");
        println!("{:?}", e);
    }
}
