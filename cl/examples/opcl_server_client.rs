use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3333")?;
    // let msg = b"set_1_hello";
    // let msg = b"set_1_world";
    // let msg = b"set_2_value";
    let msg = b"get_1_xxxxx";
    // let msg = b"get_2_xxxxx";
    stream.write(msg)?;
    stream.read(&mut [0; 128])?;
    Ok(())
}
