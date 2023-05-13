use std::path::Path;
use std::{fs, thread};
use std::time::Duration;
use cl::ocl_fs;
use cl::ocl_fs::{ocl_cache, ocl_initialize};
use cl::ocl_v5::load_dirs;

fn main() {
    ocl_initialize(true);

    let path = Path::new("./cool/node_modules");
    let mut v = vec![];
    load_dirs(&path, &mut v).expect("visit_dirs");

    println!("total {}", v.len());

    for (i, entry) in v.iter().enumerate() {
        let path = entry.path();
        let path_v = fs::read(path.as_path()).unwrap();

        match ocl_fs::write(path, path_v) {
            Ok(_) => {
                // pass
            }
            Err(_) => {
                println!("write error i: {i}")
            }
        }
    }

    thread::sleep(Duration::from_millis(2000));
    println!("Thread #main finished!");

    ocl_cache().unwrap();
}
