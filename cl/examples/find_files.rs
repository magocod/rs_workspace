use std::fs::{self, DirEntry};
use std::io;
use std::io::{Error, ErrorKind};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

type CallBack = dyn Fn(&DirEntry) -> io::Result<()>;

// https://doc.rust-lang.org/std/fs/fn.read_dir.html
fn visit_dirs(dir: &Path, cb: &CallBack) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
    Ok(())
}

fn print_dir_entry(entry: &DirEntry) -> io::Result<()> {
    let size = entry.metadata()?.size() as f64;
    let size_mb = size / (1024 * 1024) as f64;
    let path = entry.path();
    let path_str = path.to_str().ok_or(Error::new(ErrorKind::Other, "oh no"))?;
    // println!("{entry:?}");
    println!("{}", path_str);
    println!("size {size} bytes -> {} mb", size_mb);

    // if size_mb > 0.9 {
    //     println!("{entry:?}");
    //     println!("size {size} bytes -> {} mb", size_mb);
    // }

    // let file = File::open("./cl/examples/files/package-lock.json").expect("File::open");
    // println!("{file:?}");

    Ok(())
}

fn main() {
    let path = Path::new("./cl/examples/node_modules");
    visit_dirs(&path, &print_dir_entry).expect("visit_dirs");
}
