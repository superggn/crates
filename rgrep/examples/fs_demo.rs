use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::{Path, PathBuf};
fn main() -> Result<(), Error> {
    let path = Path::new("haha");
    println!("path: {:?}", &path);
    let mut file = File::open(path)?;
    let mut sss: String = "".into();
    file.read_to_string(&mut sss)?;
    println!("sss: {sss}");
    let ll: Vec<u8> = sss.into();
    println!("ll: {:?}", ll);
    Ok(())
}
