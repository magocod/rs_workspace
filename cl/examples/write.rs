use std::fs::File;
use std::io::Write;
use std::{env, fs};

fn main() {
    println!("{:?}", env::current_dir().unwrap());
    // Create a file
    let mut data_file = File::create("./cl/examples/output/data.txt").expect("creation failed");

    // Write contents to the file
    data_file
        .write("Hello, World!".as_bytes())
        .expect("write failed");

    File::create("./cl/examples/output/del.txt").expect("creation failed");
    fs::remove_file("./cl/examples/output/del.txt").unwrap();

    // File::create("./cl/examples/output/data.txt").expect("creation failed");
    // File::create("./cl/examples/output/data.txt").expect("creation failed");

    println!("Created a file data.txt");
}
