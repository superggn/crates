fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let slice = &v[..];
    println!("v: {:?}", v);
    println!("slice: {:?}", slice);
    v.pop();
    println!("v: {:?}", v);
    // println!("slice: {:?}", slice);
    v[0] = 10;
    println!("v: {:?}", v);
    // println!("slice: {:?}", slice);
}
