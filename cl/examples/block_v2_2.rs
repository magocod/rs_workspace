use cl::ocl_v2::{BlockConfig, OpenClBlock};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;

const KB_N: usize = cl::ocl_v2::LIST_SIZE;
const PIPE_MAX_PACKETS: usize = cl::ocl_v2::PIPE_MAX_PACKETS;
const PIPE_BLOCKS: usize = cl::ocl_v2::PIPE_BLOCKS;

fn main() {
    let ocl_block = OpenClBlock::new(BlockConfig {
        buffer_size: KB_N,
        pipes: KB_N,
        pipe_max_packets: PIPE_MAX_PACKETS,
    })
    .expect("OpenClBlock::new()");
    let mut pipe_blocks = ocl_block
        .generate_pipes(PIPE_BLOCKS)
        .expect("ocl_block.generate_pipes()");

    if let Some(mut pipe_block) = pipe_blocks.pop() {
        let file = File::open("./cl/examples/files/package-lock.json").expect("File::open");
        let capacity_8_kb = 8192;
        let capacity_8_mb = capacity_8_kb * 1000;
        let mut reader = BufReader::with_capacity(capacity_8_mb, file);
        let buffer = reader.fill_buf().unwrap();
        println!("buffer.len {}", buffer.len());
        // println!("input {:?}", buffer);

        pipe_block
            .enqueue_v2(buffer)
            .expect("pipe_block.enqueue_v2");
        thread::sleep(Duration::from_millis(1000));
        let _v = pipe_block.dequeue_v2().expect("pipe_block.dequeue");
        // println!("{v:?}");
        //
        // let mut data_file = File::create("./cl/examples/output/constants_mem.js").expect("creation failed");
        // // Write contents to the file
        // data_file.write(v.as_slice()).expect("write failed");
    }

    thread::sleep(Duration::from_millis(4000));

    println!("Thread #main finished!");
}
