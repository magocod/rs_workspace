use cl::ocl_fs::OclFile;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    // file system
    let f = File::open("./cl/examples/files/messages.js".to_string())?; // 2,1 kB
    let mut reader = BufReader::new(f);
    let buffer = reader.fill_buf()?;

    let path = "./cl/examples/files/messages.js".to_string();
    let mut f = OclFile::create(path)?;
    f.write(buffer)?;

    let file = OclFile::open("./cl/examples/files/messages.js".to_string())?;

    let mut reader = BufReader::new(file.clone());
    let buffer = reader.fill_buf()?;
    println!("File buffer.len() {}", buffer.len());
    // println!("{buffer:?}");

    let mut reader_b = BufReader::new(file.clone());
    let buffer_b = reader_b.fill_buf()?;
    println!("File buffer.len() {}", buffer_b.len());

    let mut reader_c = BufReader::new(file.clone());
    let buffer_c = reader_c.fill_buf()?;
    println!("File buffer.len() {}", buffer_c.len());

    Ok(())
}
