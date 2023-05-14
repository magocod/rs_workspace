#![allow(non_camel_case_types, non_snake_case)]

use cl::error::OpenClResult;
use cl::ocl_v6::{default_config, OpenClBlock, MB_1};
use opencl3::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let c = default_config();
    let mut ocl_block = OpenClBlock::new(c).expect("OpenClBlock::new()");
    ocl_block.initialize_kernel();

    let child = thread::Builder::new()
        // .stack_size(30 * 1024 * 1024)
        .spawn(move || -> OpenClResult<()> {
            // let file = File::open("./cl/examples/files/package.json").expect("File::open");
            let file = File::open("./cl/examples/files/constants.js").expect("File::open");
            // let file = File::open("./cl/examples/files/package-lock.json").expect("File::open");
            // let capacity_8_kb = 8192;
            let buff_capacity = MB_1;
            let mut reader = BufReader::with_capacity(buff_capacity, file);
            let buffer = reader.fill_buf().unwrap();
            println!("buffer.len {}", buffer.len());

            let key = ocl_block.get_block_key(buffer.len())?;
            ocl_block
                .enqueue_buffer(buffer, &key)
                .expect("ocl_block.enqueue_buffer");
            thread::sleep(Duration::from_millis(500));
            let v = ocl_block
                .dequeue_buffer(&key)
                .expect("ocl_block.dequeue_buffer");

            println!("v: {}", String::from_utf8(v.clone()).expect("from_utf8"));

            println!("output_vec len {}", v.len());
            println!("output_vec {:?}", v);

            let file = File::open("./cl/examples/files/package-lock.json").expect("File::open");
            // let capacity_8_kb = 8192;
            let buff_capacity = MB_1;
            let mut reader = BufReader::with_capacity(buff_capacity, file);
            let buffer = reader.fill_buf().unwrap();
            println!("buffer.len {}", buffer.len());

            let key = ocl_block.get_block_key(buffer.len())?;
            ocl_block
                .enqueue_buffer(buffer, &key)
                .expect("ocl_block.enqueue_buffer");
            thread::sleep(Duration::from_millis(500));
            let v = ocl_block
                .dequeue_buffer(&key)
                .expect("ocl_block.dequeue_buffer");

            println!("v: {}", String::from_utf8(v.clone()).expect("from_utf8"));

            println!("output_vec len {}", v.len());
            // println!("output_vec {:?}", v);

            println!("Thread #child finished!");
            thread::sleep(Duration::from_millis(2000));

            println!("{:#?}", ocl_block.get_global_array_map());

            Ok(())
        })
        .unwrap();

    let _ = child.join().unwrap();

    println!("Thread #main finished!");
    Ok(())
}
