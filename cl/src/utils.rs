use std::fs::DirEntry;
use std::path::Path;
use std::{fs, io};

pub fn load_dirs(dir: &Path, vec: &mut Vec<DirEntry>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                load_dirs(&path, vec)?;
            } else {
                // let size = entry.metadata().unwrap().len() as f64;
                // let size_mb = size / (1024 * 1024) as f64;
                // //
                let path = entry.path();
                let path_str = path.to_str().unwrap();

                // println!("{}", path_str);
                // println!("size {size} bytes -> {} mb", size_mb);

                vec.push(entry);

                // if size_mb > 0.9 {
                //     println!("{}", path_str);
                //     println!("size {size} bytes -> {} mb", size_mb);
                //     vec.push(entry);
                // }
                // if size_mb < 0.9 {
                //     // println!("{}", path_str);
                //     // println!("size {size} bytes -> {} mb", size_mb);
                //     vec.push(entry);
                // }
                // if size_mb < 4.0 {
                //     // println!("{}", path_str);
                //     // println!("size {size} bytes -> {} mb", size_mb);
                //     vec.push(entry);
                // }
                // if size_mb < 0.9 && size > 1024 as f64 {
                //     // println!("{}", path_str);
                //     // println!("size {size} bytes -> {} mb", size_mb);
                //     vec.push(entry);
                // }
                // if size < 1024 as f64 {
                //     // println!("{}", path_str);
                //     // println!("size {size} bytes -> {} mb", size_mb);
                //     vec.push(entry);
                // }
            }
        }
    }
    Ok(())
}

pub fn load_all_from_dirs(dir: &Path, vec: &mut Vec<DirEntry>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                load_dirs(&path, vec)?;
            } else {
                vec.push(entry);
            }
        }
    }
    Ok(())
}

pub fn load_dirs_string(dir: &Path, vec: &mut Vec<String>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                load_dirs_string(&path, vec)?;
            } else {
                // let size = entry.metadata().unwrap().len() as f64;
                // let size_mb = size / (1024 * 1024) as f64;
                // //
                let path = entry.path();
                let path_str = path.to_str().unwrap();

                // println!("{}", path_str);
                // println!("size {size} bytes -> {} mb", size_mb);

                // vec.push(entry);
                let path = entry.path();
                let path_str = path.to_str().unwrap();
                vec.push(path_str.to_string());

                // if size_mb > 0.9 {
                //     println!("{}", path_str);
                //     println!("size {size} bytes -> {} mb", size_mb);
                //     vec.push(entry);
                // }
                // if size_mb < 0.9 {
                //     // println!("{}", path_str);
                //     // println!("size {size} bytes -> {} mb", size_mb);
                //     vec.push(entry);
                // }
                // if size_mb < 4.0 {
                //     // println!("{}", path_str);
                //     // println!("size {size} bytes -> {} mb", size_mb);
                //     vec.push(entry);
                // }
                // if size_mb < 0.9 && size > 1024 as f64 {
                //     // println!("{}", path_str);
                //     // println!("size {size} bytes -> {} mb", size_mb);
                //     vec.push(entry);
                // }
                // if size < 1024 as f64 {
                //     // println!("{}", path_str);
                //     // println!("size {size} bytes -> {} mb", size_mb);
                //     vec.push(entry);
                // }
            }
        }
    }
    Ok(())
}

pub fn load_all_from_dirs_string(dir: &Path, vec: &mut Vec<String>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                load_dirs_string(&path, vec)?;
            } else {
                let path = entry.path();
                let path_str = path.to_str().unwrap();
                vec.push(path_str.to_string());
            }
        }
    }
    Ok(())
}
