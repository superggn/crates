use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let db = sled::open("/tmp/welcome-to-sled")?;

    // insert and get, similar to std's BTreeMap
    let old_value = db.insert("key", "value")?;
    println!("old_value: {:?}", old_value);

    assert_eq!(db.get(&"key")?, Some(sled::IVec::from("value")),);

    // range queries
    println!("kv_result: ===");
    for kv_result in db.range("key_1".."key_9") {
        println!("{:?}", kv_result?);
    }

    // deletion
    let old_value = db.remove(&"key")?;
    println!("old_value: {:?}", old_value);

    // atomic compare and swap
    db.compare_and_swap("key", Some("current_value"), Some("new_value"))?;

    // block until all operations are stable on disk
    // (flush_async also available to get a Future)
    db.flush()?;
    Ok(())
}
