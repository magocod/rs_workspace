use cl::ocl_fs_2;
use cl::ocl_fs_2::{ocl_cache, ocl_initialize, ocl_summary};
use cl::utils::{load_all_from_dirs};
use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

fn main() {
    ocl_initialize();

    let path = Path::new("./cool/node_modules");
    let mut v = vec![];
    // load_dirs(&path, &mut v).expect("visit_dirs");
    load_all_from_dirs(&path, &mut v).expect("visit_dirs");

    println!("total {}", v.len());

    let mut total_size = 0.0;

    for (i, entry) in v.iter().enumerate() {
        let path = entry.path();
        let path_v = fs::read(path.as_path()).unwrap();

        // if path_v.len() > 1024 {
        //     println!("{path:?}, len {}", path_v.len())
        // }

        total_size += path_v.len() as f64;

        println!("{path:?}");

        match ocl_fs_2::write(path.as_path(), path_v.as_slice()) {
            Ok(_) => {
                // pass
            }
            Err(e) => {
                println!("write error i: {i}");
                println!("path: len {} {path:?}", path_v.len());
                println!("err: {e:?}");
            }
        }
    }

    thread::sleep(Duration::from_millis(2000));

    println!("Thread #main finished!");

    ocl_cache().unwrap();
    ocl_summary().unwrap();
    println!("total mb {}", total_size / (1024 * 1024) as f64);
    println!("total {}", v.len());
}
