use cl::ocl_v1::OpenClBlock;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;

const THREADS: u8 = 1;
const BLOCK_PER_THREAD: u8 = 64;

fn main() {
    let threads: Vec<_> = (0..THREADS)
        .map(|i| {
            thread::spawn(move || {
                println!("Thread #{i} started!");
                let mut ocl_blocks: Vec<OpenClBlock> = vec![];
                for index in 0..BLOCK_PER_THREAD {
                    println!("thread {i} BLOCK {index}");
                    let ocl_block = OpenClBlock::new().expect("OpenClBlock::new()");
                    ocl_blocks.push(ocl_block);

                    // let file = File::open("./cl/examples/files/constants.js").expect("File::open");
                    // let capacity_8_kb = 8192;
                    // // let capacity_8_mb = capacity_8_kb * 1000;
                    // let mut reader = BufReader::with_capacity(capacity_8_kb, file);
                    // let buffer = reader.fill_buf().unwrap();
                    // println!("buffer.len {}", buffer.len());
                    // println!("input {:?}", buffer);
                }
                // let ocl_block = OpenClBlock::new().expect("OpenClBlock::new()");
                //
                // let file = File::open("./cl/examples/files/constants.js").expect("File::open");
                // let capacity_8_kb = 8192;
                // // let capacity_8_mb = capacity_8_kb * 1000;
                // let mut reader = BufReader::with_capacity(capacity_8_kb, file);
                // let buffer = reader.fill_buf().unwrap();
                // println!("buffer.len {}", buffer.len());
                // // println!("input {:?}", buffer);

                // ocl_block.enqueue(buffer).expect("ocl_block.enqueue");
                // thread::sleep(Duration::from_millis(2000));
                // let _ = ocl_block.dequeue().expect("ocl_block.dequeue");
                // println!("{v:?}");
                // let mut data_file = File::create("./cl/examples/output/constants_mem.js").expect("creation failed");
                //
                // // Write contents to the file
                // data_file.write(v.as_slice()).expect("write failed");

                thread::sleep(Duration::from_millis(2000));
                println!("Thread #{i} finished!");
            })
        })
        .collect();

    for handle in threads {
        handle.join().unwrap();
    }

    thread::sleep(Duration::from_millis(4000));
    println!("Thread #main finished!");
}
