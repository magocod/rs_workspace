use cl::ocl_fs_2;
use cl::ocl_fs_2::{ocl_cache, ocl_initialize};
use cl::utils::load_dirs;
use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

fn main() {
    ocl_initialize();

    let path = Path::new("./cool/node_modules");
    let mut v = vec![];
    load_dirs(&path, &mut v).expect("visit_dirs");

    println!("total {}", v.len());

    for (i, entry) in v.iter().enumerate() {
        let path = entry.path();
        let path_v = fs::read(path.as_path()).unwrap();

        if path_v.len() > 1024 {
            println!("{path:?}, len {}", path_v.len())
        }

        match ocl_fs_2::write(path, path_v) {
            Ok(_) => {
                // pass
            }
            Err(e) => {
                println!("write error i: {i}");
                println!("write error i: {e:?}");
            }
        }
    }

    thread::sleep(Duration::from_millis(2000));
    println!("Thread #main finished!");

    ocl_cache().unwrap();
}
