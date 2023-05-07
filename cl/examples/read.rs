use cl::ocl_v2::KB_1;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};

fn main() -> std::io::Result<()> {
    // let file = File::open("./cl/examples/files/info.txt")?; // 50 bytes
    // let file = File::open("./cl/examples/files/index.html")?; // 169 bytes
    // let file = File::open("./cl/examples/files/constants.js")?; // 979 bytes
    let file = File::open("./cl/examples/files/messages.js")?; // 2,1 kB
                                                               // let file = File::open("./cl/examples/files/main.js")?; // 567,7 kB
                                                               // let file = File::open("./cl/examples/files/libaho_corasick-82cd3436e40cc54e.rmeta")?; // 1,1 MB
                                                               // let file = File::open("./cl/examples/files/libaho_corasick-d76e4ca2c863053c.rlib")?; // 5,7 MB
                                                               // let file = File::open("./cl/examples/files/libasync_stream_impl-bf423722f4ddc636.so")?; // 19,7 MB
                                                               // let file = File::open("./cl/examples/files/libbindgen-c67fa65f4f8a335c.rlib")?; // 40,4 MB
                                                               // let file = File::open("./cl/examples/files/0.pack")?; // 79.09 MB

    // let mut reader = BufReader::new(file);
    let capacity_8_kb = 8192;
    let capacity_8_mb = capacity_8_kb * 1000;
    let capacity_128_mb = capacity_8_mb * 16;
    // println!("capacity_8_mb {capacity_8_mb}");
    // println!("capacity_128_mb {capacity_128_mb}");
    let mut reader = BufReader::with_capacity(capacity_128_mb, file);

    // for line in reader.lines() {
    //     let line = line?;
    //     println!("{}", line);
    // }

    let buffer = reader.fill_buf().unwrap();

    // work with buffer
    println!("{}", buffer.len());
    // println!("{buffer:?}");

    let c = buffer.chunks(KB_1);
    // println!("{c:?}");

    println!("ChunksMut.len {}", c.len());

    for chunk in c {
        println!("chunk: {}", chunk.len());
        // println!("chunk: {chunk:?}");
        // for elem in chunk.iter() {
        //     println!("elem {elem}");
        // }
    }

    // let mut data_file = File::create("./cl/examples/output/messages.js").expect("creation failed");
    //
    // // Write contents to the file
    // data_file.write(buffer).expect("write failed");

    Ok(())
}
