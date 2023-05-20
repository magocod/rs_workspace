use std::io;

fn main() {
    let mut reader: &[u8] = b"hello";
    let mut writer: Vec<u8> = Vec::with_capacity(1024);

    let len = io::copy(&mut reader, &mut writer).unwrap();

    // assert_eq!(&b"hello"[..], &writer[..]);
    println!("{len}");
    println!("{writer:?}");
}
