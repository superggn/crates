use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    // 从文件读取
    let file = File::open("example.txt")?;
    let mut buf_reader = BufReader::new(file);

    // 从 TCP 流写入
    let stream = TcpStream::connect("127.0.0.1:8080")?;
    let mut buf_writer = BufWriter::new(stream);

    // 从 Vec<u8> 读取
    let data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let cursor = io::Cursor::new(data); // 使用 Cursor 来允许读写 Vec<u8>
    let mut buf_reader_from_vec = BufReader::new(cursor);

    Ok(())
}
