#![allow(non_camel_case_types, non_snake_case)]

use cl::ocl_v5::{OpenClBlock, MB_1};
use opencl3::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let mut ocl_block = OpenClBlock::new().expect("OpenClBlock::new()");
    let vector_add_kernel = ocl_block.create_vector_add_kernel();
    let vector_extract_kernel = ocl_block.create_vector_extract_kernel();

    let child = thread::Builder::new()
        // .stack_size(30 * 1024 * 1024)
        .spawn(move || -> Result<()> {
            // let file = File::open("./cl/examples/files/package.json").expect("File::open");
            let file = File::open("./cl/examples/files/constants.js").expect("File::open");
            // let file = File::open("./cl/examples/files/package-lock.json").expect("File::open");
            // let capacity_8_kb = 8192;
            let buff_capacity = MB_1;
            let mut reader = BufReader::with_capacity(buff_capacity, file);
            let buffer = reader.fill_buf().unwrap();
            println!("buffer.len {}", buffer.len());

            let index = 0;
            ocl_block
                .enqueue_buffer(&vector_add_kernel, buffer, index)
                .expect("ocl_block.enqueue_buffer");
            thread::sleep(Duration::from_millis(500));
            ocl_block
                .dequeue_buffer(&vector_extract_kernel, index)
                .expect("ocl_block.dequeue_buffer");

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
