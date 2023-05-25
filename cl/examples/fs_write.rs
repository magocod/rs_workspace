use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    println!("{:?}", std::env::current_dir());
    let path = "./tmp/data.txt";

    let mut file = File::create(path)?;
    println!("{file:?}");

    file.write(b"a")?;
    file.write(b"b")?;
    file.write_all(b"c")?;
    file.write_all(b"d")?;

    let mut file = OpenOptions::new().write(true).open(path)?;
    println!("{file:?}");

    file.write(b"1").expect("TODO: panic message");
    file.write(b"2").expect("TODO: panic message");

    let mut file = File::create(path)?;
    println!("{file:?}");

    file.write(b"A")?;

    Ok(())
}
