use std::mem;

fn main() {
    let mut x = "hello world".to_string();
    let mut y = "goodbye world".to_string();

    mem::swap(&mut x, &mut y);

    assert_eq!("goodbye world", x);
    assert_eq!("hello world", y);
    println!("assert done!")
}
