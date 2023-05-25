use flate2::write::{GzEncoder, ZlibEncoder};
use flate2::Compression;
use std::io::prelude::*;

fn main() {
    // Vec<u8> implements Write, assigning the compressed bytes of sample string
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(b"Hello World").unwrap();
    let compressed = e.finish().unwrap();
    println!("zlip compressed {compressed:?}");
    println!("{:?}", b"Hello World");

    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(b"Hello World").unwrap();
    let compressed = e.finish().unwrap();
    println!("gz compressed {compressed:?}");
    println!("{:?}", b"Hello World");
}
