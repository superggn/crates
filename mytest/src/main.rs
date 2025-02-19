use futures::StreamExt;
use tokio::net::TcpListener;
// use tokio::prelude::*;
use yamux::{Config, Session};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    while let Ok((socket, _)) = listener.accept().await {
        let session = Session::new(Config::default(), socket);

        tokio::spawn(handle_client(session));
    }

    Ok(())
}

async fn handle_client(mut session: Session<tokio::net::TcpStream>) {
    while let Some(stream) = session.next().await {
        match stream {
            Ok(mut stream) => {
                let _ = tokio::spawn(async move {
                    let mut buf = [0; 1024];
                    loop {
                        let n = match stream.read(&mut buf).await {
                            Ok(0) => return, // connection closed
                            Ok(n) => n,
                            Err(_) => return, // error
                        };
                        if let Err(_) = stream.write_all(&buf[..n]).await {
                            return; // error writing back
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept stream: {}", e);
            }
        }
    }
}
