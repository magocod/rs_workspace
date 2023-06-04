use std::{env, fs};

fn main() {
    let e = env::current_dir().unwrap();
    let p = e.as_path().join("tmp/a.txt");

    fs::remove_dir_all(p).unwrap();
}
