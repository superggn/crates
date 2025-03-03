fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("OUT_DIR is: {}", std::env::var("OUT_DIR").unwrap());
}
