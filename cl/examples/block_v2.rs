use cl::ocl_v2::{BlockConfig, OpenClBlock};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;

const THREADS: u8 = 1;

// CONFIG A =
// buffer_capacity: 1 kb - pipe_capacity: 8 kb
// memory_store: 5.12 MB - memory_required: 2.560 GB
// pipe_blocks = 640

const KB_N: usize = cl::ocl_v2::LIST_SIZE;
const PIPE_MAX_PACKETS: usize = cl::ocl_v2::PIPE_MAX_PACKETS;
const PIPE_BLOCKS: usize = cl::ocl_v2::PIPE_BLOCKS;

fn main() {
    let threads: Vec<_> = (0..THREADS)
        .map(|i| {
            thread::spawn(move || {
                println!("Thread #{i} started!");
                let ocl_block = OpenClBlock::new(BlockConfig {
                    buffer_size: KB_N,
                    pipes: KB_N,
                    pipe_max_packets: PIPE_MAX_PACKETS,
                })
                .expect("OpenClBlock::new()");
                let mut pipe_blocks = ocl_block
                    .generate_pipes(PIPE_BLOCKS)
                    .expect("ocl_block.generate_pipes()");

                // if let Some(pipe_block) = pipe_blocks.pop() {
                //     let file = File::open("./cl/examples/files/constants.js").expect("File::open");
                //     let capacity_8_kb = 8192;
                //     // let capacity_8_mb = capacity_8_kb * 1000;
                //     let mut reader = BufReader::with_capacity(capacity_8_kb, file);
                //     let buffer = reader.fill_buf().unwrap();
                //     println!("buffer.len {}", buffer.len());
                //     // println!("input {:?}", buffer);
                //
                //     pipe_block.enqueue(buffer).expect("pipe_block.enqueue");
                //     thread::sleep(Duration::from_millis(1000));
                //     let v = pipe_block.dequeue().expect("pipe_block.dequeue");
                //     // println!("{v:?}");
                //
                //     let mut data_file = File::create("./cl/examples/output/constants_mem.js").expect("creation failed");
                //     // Write contents to the file
                //     data_file.write(v.as_slice()).expect("write failed");
                // }

                thread::sleep(Duration::from_millis(4000));
                println!("Thread #{i} finished!");
            })
        })
        .collect();

    for handle in threads {
        handle.join().unwrap();
    }

    thread::sleep(Duration::from_millis(1000));
    println!("Thread #main finished!");
}
