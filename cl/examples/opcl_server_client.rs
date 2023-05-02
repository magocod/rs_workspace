use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3333")?;
    // let msg = b"set_hello";
    // let msg = b"set_world";
    let msg = b"get_hello";
    stream.write(msg)?;
    stream.read(&mut [0; 128])?;
    Ok(())
}
