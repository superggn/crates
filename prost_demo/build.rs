use std::io::Result;
fn main() -> Result<()> {
    // prost_build::Config::new()
    //     .out_dir("src/msg/")
    //     .compile_protos(&["src/msg/msg.proto"], &["."])
    //     .unwrap();
    prost_build::compile_protos(&["src/msg/msg.proto"], &["."]).unwrap();
    Ok(())
}
