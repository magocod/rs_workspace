use cl::ocl_fs_2;
use cl::ocl_fs_2::{ocl_cache, ocl_cache_map, ocl_initialize, ocl_summary};
use cl::utils::load_all_from_dirs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{fs, thread};

fn main() {
    ocl_initialize();

    let path = Path::new("./tmp/ts_express_node_modules");
    let mut v = vec![];
    // load_dirs(&path, &mut v).expect("visit_dirs");
    load_all_from_dirs(&path, &mut v).expect("visit_dirs");

    println!("total {}", v.len());

    let mut total_size = 0.0;

    for (_, entry) in v.iter().enumerate() {
        let path = entry.path();
        let path_v = fs::read(path.as_path()).unwrap();

        // if path_v.len() > 1024 {
        //     println!("{path:?}, len {}", path_v.len())
        // }

        total_size += path_v.len() as f64;

        // println!("{path:?}");

        match ocl_fs_2::write(path.as_path(), path_v.as_slice()) {
            Ok(_) => {
                // pass
            }
            Err(_) => {
                // pass
            } // Err(e) => {
              //     println!("write error i: {i}");
              //     println!("path: len {} {path:?}", path_v.len());
              //     println!("err: {e:?}");
              // }
        }
    }

    thread::sleep(Duration::from_millis(2000));

    println!("load in gpu completed");
    let map = ocl_cache_map();

    let tmp_str = "./tmp";

    for (key, _) in map {
        let key_2 = &key[tmp_str.len()..key.len()];
        let key_3 = format!("./tmp/vram{key_2}");
        let mut path_key = PathBuf::from(key_3.as_str());
        path_key.pop();

        let buffer = ocl_fs_2::read(key).unwrap();

        match fs::create_dir_all(path_key) {
            Ok(_) => {
                match fs::write(key_3, buffer) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                };
            }
            Err(e) => {
                println!("{}", e);
            }
        };
    }

    println!("Thread #main finished!");

    // ocl_cache().unwrap();
    ocl_summary().unwrap();
    println!("disk total mb {}", total_size / (1024 * 1024) as f64);
    println!("disk total {}", v.len());
}
