use std::fs::File;
use std::io::Write;

fn main() {
    // Create a file
    let mut data_file = File::create("./cl/examples/output/data.txt").expect("creation failed");

    // Write contents to the file
    data_file
        .write("Hello, World!".as_bytes())
        .expect("write failed");

    println!("Created a file data.txt");
}
