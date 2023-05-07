use cl::ocl_v3::OpenClBlock;
use opencl3::types::cl_int;
use std::io::prelude::*;
use std::io::Result;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream, ocl: &mut OpenClBlock) {
    let mut buffer = [0; 11];
    stream.read(&mut buffer).unwrap();
    println!("buffer {buffer:?}");

    let str = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {str}");

    let parts = str.split("_").collect::<Vec<&str>>();
    // println!("{parts:?}");
    let index: u32 = parts[1].parse().unwrap();

    if str.contains("set") {
        ocl.enqueue_buffer(&buffer, index as cl_int)
            .expect("ocl.enqueue_buffer");
    } else {
        let v = ocl
            .dequeue_buffer(index as cl_int)
            .expect("ocl.dequeue_buffer");
        println!("v {v:?}")
    }
}

pub fn create_listener(addr: String) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    let mut ocl_block = OpenClBlock::new().expect("OpenClBlock::new");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut ocl_block);
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("start");
    create_listener(String::from("127.0.0.1:3333"))?;
    println!("end");
    Ok(())
}
