use cl::ocl_v2::{BlockConfig, OpenClBlock, PipeBlock, KB_N, PIPE_BLOCKS, PIPE_MAX_PACKETS};
use std::io::prelude::*;
use std::io::Result;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream, pipe: &mut PipeBlock<'_>) {
    let mut buffer = [0; 9];
    stream.read(&mut buffer).unwrap();
    println!("buffer {buffer:?}");

    let str = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {str}");
    if str.contains("set") {
        pipe.enqueue_v2(&buffer).expect("pipe.enqueue_v2");
    } else {
        let v = pipe.dequeue_v2().expect("pipe.dequeue_v2");
        println!("pipe {v:?}")
    }
}

pub fn create_listener(addr: String) -> Result<()> {
    let listener = TcpListener::bind(addr)?;

    let ocl_block = OpenClBlock::new(BlockConfig {
        buffer_size: KB_N,
        pipes: KB_N,
        pipe_max_packets: PIPE_MAX_PACKETS,
    })
    .expect("OpenClBlock::new");
    let mut pipe_blocks = ocl_block
        .generate_pipes(PIPE_BLOCKS)
        .expect("ocl_block.generate_pipes()");

    let p = &mut pipe_blocks[0];
    // println!("p {p:?}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, p);
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("start");
    create_listener(String::from("127.0.0.1:3333"))?;
    println!("end");
    Ok(())
}
