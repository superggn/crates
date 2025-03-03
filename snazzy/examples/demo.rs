use snazzy::create_large_shirt;
fn main() {
    println!("start!");
    let shirt = create_large_shirt("red".into());
    println!("shirt: {:?}", shirt);
    println!("finish!");
}
