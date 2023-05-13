use cl::ocl_fs;
use cl::ocl_fs::{ocl_cache, OclFile};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    let path = "./cl/examples/files/info.txt";
    // file system
    let file = File::open(path)?; // 2,1 kB

    let mut reader = BufReader::new(file);
    let buffer = reader.fill_buf()?;
    // work with buffer
    println!("File buffer.len() {}", buffer.len());
    // println!("{buffer:?}");
    // println!("{:?}", String::from_utf8_lossy(buffer));

    let v = fs::read(path)?;
    println!("fs::read v.len() {}", v.len());

    let read_str = fs::read_to_string(path).unwrap();
    println!("fs::read_to_string {}", read_str);

    // opencl
    let mut f = OclFile::create(path)?;
    f.write(buffer)?;

    let file = OclFile::open(path)?;

    let mut reader = BufReader::new(file);
    let buffer = reader.fill_buf()?;
    // work with buffer
    println!("oclFile buffer.len() {}", buffer.len());
    // println!("{buffer:?}");
    // println!("{:?}", String::from_utf8_lossy(buffer));

    let v = ocl_fs::read(path)?;
    println!("ocl_fs::read v.len() {}", v.len());

    let read_str = ocl_fs::read_to_string(path).unwrap();
    println!("ocl_fs::read_to_string {}", read_str);

    ocl_cache()?;

    Ok(())
}
