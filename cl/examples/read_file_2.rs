use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    // file system
    let file = File::open("./cl/examples/files/messages.js".to_string())?; // 2,1 kB
                                                                           // let file = File::open("./cl/examples/files/void.txt")?; // 0 kB

    let mut reader = BufReader::new(file.try_clone()?);
    let buffer = reader.fill_buf()?;
    println!("File buffer.len() {}", buffer.len());
    // println!("{buffer:?}");

    let mut reader_b = BufReader::new(file.try_clone()?);
    let buffer_b = reader_b.fill_buf()?;
    println!("File buffer.len() {}", buffer_b.len());

    let mut reader_c = BufReader::new(file.try_clone()?);
    let buffer_c = reader_c.fill_buf()?;
    println!("File buffer.len() {}", buffer_c.len());

    Ok(())
}
