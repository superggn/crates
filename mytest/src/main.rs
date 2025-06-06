fn main() {
    let buf = [0; 10];
    let buf_slice = &buf[..];
    println!("buf: {:?}", buf);
    println!("buf_slice: {:?}", buf_slice);
    println!("buf: {:?}", buf);
    println!("buf_slice: {:?}", buf_slice);
    let buf_slice_2 = &buf[..];
}
