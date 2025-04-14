use bytes::{Buf, BufMut, BytesMut};

fn main() {
    // Step 1: 创建一个 BytesMut 并填充数据
    let mut buf = BytesMut::new();
    buf.put(&b"Hello, world!"[..]);

    println!("buf: {:?}", buf);
    println!("buf pointer: {:p}", &buf);
    println!("buf.len(): {}", buf.len());
    println!("buf.capacity(): {}", buf.capacity());
    println!("=========================");

    // Step 2: 使用 reserve 保证缓冲区有足够的空间
    buf.reserve(10); // 这里你可以调整 reserve 的大小，看看有什么变化
    println!("buf.reserve(10);");
    println!("buf: {:?}", buf);
    println!("buf.len(): {}", buf.len());
    println!("buf.capacity(): {}", buf.capacity());
    println!("=========================");

    // Step 3: 使用 advance_mut(cnt) 移动缓冲区的有效数据部分, 向后扩cnt位
    unsafe {
        buf.advance_mut(7); // 这个操作后，缓冲区会变成 "Hello, world!......"
    }
    println!("buf.advance_mut(7)");
    println!("buf: {:?}", buf);
    println!("buf.len(): {}", buf.len());
    println!("buf.capacity(): {}", buf.capacity());
    println!("=========================");

    // Step 4: 使用 split_to 切割缓冲区
    let mut part = buf.split_to(5); // 只取前5个字节
    println!("let part = buf.split_to(5)");
    println!("part: {:?}", part);
    println!("part.len(): {}", part.len());
    println!("part.capacity(): {}", part.capacity());
    part.reserve(10);
    println!("part.reserve(10);");
    println!("part: {:?}", part);
    println!("part.len(): {}", part.len());
    println!("part.capacity(): {}", part.capacity());
    println!("-------------------------");
    println!("buf: {:?}", buf);
    println!("buf.len(): {}", buf.len());
    println!("buf.capacity(): {}", buf.capacity());
    println!("=========================");

    // Step 5: 将数据冻结，转为不可变的 Bytes
    let frozen = buf.freeze();
    println!("let frozen = buf.freeze();");
    println!("buf is consumed");
    println!("frozen is a Bytes (not mut): {:?}", frozen);
    println!("frozen.len(): {}", frozen.len());
    println!("frozen doesn't have capacity");
    println!("=========================");

    // Step 6: 使用 put 添加更多数据
    let mut buf2 = BytesMut::new();
    println!("buf2: {:?}", buf2);
    println!("buf2.len(): {}", buf2.len());
    println!("buf2.capacity(): {}", buf2.capacity());
    buf2.put(&b" Rust!"[..]);
    println!("buf2.put(&b\" Rust!\"[..]);");
    println!("buf2: {:?}", buf2);
    println!("buf2.len(): {}", buf2.len());
    println!("buf2.capacity(): {}", buf2.capacity());
    println!("=========================");

    // Step 7: 使用 copy_to_bytes 将数据从 BytesMut 复制到 Bytes
    let copied = buf2.copy_to_bytes(5); // 只复制前5个字节
    println!("let copied = buf2.copy_to_bytes(5);");
    println!("buf2: {:?}", buf2);
    println!("Copied Bytes: {:?}", copied);
    println!("buf2.len(): {}", buf2.len());
    println!("buf2.capacity(): {}", buf2.capacity());
}
