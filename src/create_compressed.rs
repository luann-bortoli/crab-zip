use std::fs::File;
use std::io::prelude::*;
pub fn create_compressed() -> std::io::Result<()> {
    let mut file = File::create("text.txt")?;
    file.write_all(b"Hello World")?;
    let f = File::open("text.txt")?;
    Ok(())
}