use std::{env, fs, io};
use std::path::Path;

fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        let r = fs::read_dir(dir)?;
        // println!("ReadDir {:?}", r);
        for entry in r {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                println!("{entry:?}");
                visit_dirs(&path)?;
            } else {
                println!("{entry:?}");
            }
        }
    }
    Ok(())
}

fn main() {
    let e = env::current_dir().unwrap();
    let p = e.as_path().join("tmp").join("directory");

    println!("e {:?}", e);
    println!("p {:?}", p);
    let _ = visit_dirs(p.as_path()).unwrap();


    for v in Path::new("/tmp/foo/bar.txt").iter() {
        println!("{v:?}, v")
    }
}