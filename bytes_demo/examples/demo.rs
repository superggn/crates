use bytes::{BufMut, BytesMut};

fn main() {
    let mut buf = BytesMut::with_capacity(10);

    // 写入数据
    buf.put_u8(0x42);
    buf.put_u16(0x1234);
    buf.put(&b"hello"[..]);

    println!("{:?}", buf); // [66, 18, 52, 104, 101, 108, 108, 111]
}
