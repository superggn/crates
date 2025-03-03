use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/items.proto"], &["src/"])?;
    println!(
        "cargo:warning=OUT_DIR is: {}",
        std::env::var("OUT_DIR").unwrap()
    );
    Ok(())
}
