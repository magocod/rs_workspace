use cl::ocl_fs_3;
use cl::ocl_fs_3::{ocl_cache, ocl_initialize, ocl_summary};
use cl::utils::load_all_from_dirs_string;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use std::{fs, thread};

const TOTAL_THREADS: usize = 7;

fn main() {
    ocl_initialize();

    let path = Path::new("./cool/node_modules");
    // let path = Path::new("../cool/node_modules");
    let mut v = vec![];
    // load_dirs(&path, &mut v).expect("visit_dirs");
    // load_all_from_dirs(&path, &mut v).expect("visit_dirs");
    load_all_from_dirs_string(&path, &mut v).expect("visit_dirs");
    let total_files = v.len();
    let total_per_thread = total_files / TOTAL_THREADS;
    println!("total {} {}", total_files, total_per_thread);

    let chunks = v.chunks(total_per_thread);

    let mut total_threads = TOTAL_THREADS;
    if chunks.len() != total_threads {
        total_threads += 1;
    }

    let c = v
        .chunks(total_per_thread)
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<String>>>();
    let vec_of_vecs = Arc::new(c);
    println!("chunks {}", vec_of_vecs.len());

    println!("total_threads {total_threads}");

    let threads: Vec<_> = (0..total_threads)
        .map(|i| {
            let chunks_ref = vec_of_vecs.clone();
            thread::spawn(move || {
                println!("Thread #{i} started!");
                // println!("values {:?}", chunks_ref[i].len());

                for (i, path) in chunks_ref[i].iter().enumerate() {
                    let path_v = fs::read(path.as_str()).unwrap();

                    match ocl_fs_3::write(path.as_str(), path_v.as_slice()) {
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

                println!("Thread #{i} finished!");
            })
        })
        .collect();

    for handle in threads {
        handle.join().unwrap();
    }

    thread::sleep(Duration::from_millis(1000));
    println!("Thread #main finished!");
    ocl_cache().unwrap();
    ocl_summary().unwrap();
    println!("total {}", v.len());
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         main();
//         assert_eq!(1, 1);
//     }
// }
