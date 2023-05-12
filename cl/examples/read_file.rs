use cl::ocl_fs::{ocl_cache, OclFile};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    // file system
    let file = File::open("./cl/examples/files/messages.js".to_string())?; // 2,1 kB
                                                                           // let file = File::open("./cl/examples/files/void.txt")?; // 0 kB

    let mut reader = BufReader::new(file);
    let buffer = reader.fill_buf()?;
    // work with buffer
    println!("File buffer.len() {}", buffer.len());
    // println!("{buffer:?}");
    // println!("{:?}", String::from_utf8_lossy(buffer));

    // opencl
    let path = "./cl/examples/files/messages.js".to_string();
    let mut f = OclFile::create(path)?;
    f.write(buffer)?;

    let file = OclFile::open("./cl/examples/files/messages.js".to_string())?;

    let mut reader = BufReader::new(file);
    let buffer = reader.fill_buf()?;
    // work with buffer
    println!("oclFile buffer.len() {}", buffer.len());
    // println!("{buffer:?}");
    // println!("{:?}", String::from_utf8_lossy(buffer));

    ocl_cache()?;

    Ok(())
}
