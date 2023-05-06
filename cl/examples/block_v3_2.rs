#![allow(non_camel_case_types, non_snake_case)]

use cl::ocl_v3::{visit_dirs, OpenClBlock, MB_1, TOTAL_GLOBAL_ARRAY};
use opencl3::types::cl_int;
use opencl3::Result;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let mut ocl_block = OpenClBlock::new().expect("OpenClBlock::new()");
    let child = thread::Builder::new()
        .stack_size(30 * 1024 * 1024)
        .spawn(move || -> Result<()> {
            let path = Path::new("./cl/examples/node_modules");

            let mut dirs: Vec<DirEntry> = Vec::with_capacity(4500);
            visit_dirs(path, &mut dirs).expect("visit_dirs");
            println!("dirs len {}", dirs.len());

            for (index, dir) in dirs.iter().enumerate() {
                if index == TOTAL_GLOBAL_ARRAY {
                    println!("index {index}");
                    break;
                }
                let file = File::open(dir.path()).expect("File::open");
                let buff_capacity = MB_1;
                let mut reader = BufReader::with_capacity(buff_capacity, file);
                let mut buffer = reader.fill_buf().unwrap();
                ocl_block
                    .enqueue_buffer(buffer, index as cl_int)
                    .expect("ocl_block.enqueue_buffer");
                buffer.consume(buffer.len());
            }

            // for i in 8..12 {
            //     ocl_block
            //         .dequeue_buffer( i as cl_int)
            //         .expect("ocl_block.enqueue_buffer");
            // }

            // let file = File::open("./cl/examples/files/package.json").expect("File::open");
            // let file = File::open("./cl/examples/files/package-lock.json").expect("File::open");
            // // let capacity_8_kb = 8192;
            // let buff_capacity = MB_1;
            // let mut reader = BufReader::with_capacity(buff_capacity, file);
            // let buffer = reader.fill_buf().unwrap();
            // println!("buffer.len {}", buffer.len());
            //
            // let index = 1;
            // ocl_block
            //     .enqueue_buffer(buffer, index)
            //     .expect("ocl_block.enqueue_buffer");
            // thread::sleep(Duration::from_millis(500));
            // ocl_block
            //     .dequeue_buffer(index)
            //     .expect("ocl_block.dequeue_buffer");

            // let mut data_file = File::create("./cl/examples/output/constants_mem.js").expect("creation failed");
            // // Write contents to the file
            // data_file.write(v.as_slice()).expect("write failed");

            println!("Thread #child finished!");
            thread::sleep(Duration::from_millis(2000));

            Ok(())
        })
        .unwrap();

    let _ = child.join().unwrap();

    println!("Thread #main finished!");
    Ok(())
}
