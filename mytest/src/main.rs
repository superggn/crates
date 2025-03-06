use bytes::{Buf, BufMut, Bytes, BytesMut};

fn main() {
    let mut buf = Bytes::from("hello world");
    println!("{:?}", buf); // "hello world"
    println!("{:?}", buf.len());
    println!("====================");

    buf.advance(6);
    println!("{:?}", buf); // "world"
    println!("{:?}", buf.len());
    println!("====================");

    let mut buf_mut = BytesMut::from("hello world");
    println!("{:?}", buf_mut); // "hello world"
    println!("{:?}", buf_mut.len());
    println!("{:?}", buf_mut.capacity());
    println!("====================");
    buf_mut.reserve(10);
    println!("{:?}", buf_mut); // "hello world"
    println!("{:?}", buf_mut.len());
    println!("{:?}", buf_mut.capacity());
    println!("====================");
    unsafe {
        buf_mut.advance_mut(6);
    }
    println!("{:?}", buf_mut); // 剩余的 BytesMut 仍然是 "world"，但前面的数据可能被覆盖
    println!("{:?}", buf_mut.len());
    println!("{:?}", buf_mut.capacity());
    buf_mut.advance(6);
    println!("{:?}", buf_mut);
    println!("{:?}", buf_mut.len());
    println!("{:?}", buf_mut.capacity());
}
