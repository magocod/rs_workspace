use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

fn main() -> io::Result<()> {
    let path = "./tmp/read.txt";

    let f = File::open(path)?;
    println!("{f:?}");

    let mut reader = BufReader::new(f);
    let buffer = reader.fill_buf()?;

    // work with buffer
    println!("File buffer.len() {}", buffer.len());
    println!("{buffer:?}");

    // open seek

    let mut f = File::open(path)?;
    println!("{f:?}");

    // move the cursor 42 bytes from the start of the file
    f.seek(SeekFrom::Start(1))?;

    let mut reader = BufReader::new(f);
    let buffer = reader.fill_buf()?;

    // work with buffer
    println!("File buffer.len() {}", buffer.len());
    println!("{buffer:?}");

    Ok(())
}
