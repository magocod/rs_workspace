use std::fs;

// ln -s /codes/v.cpp /codes/link.cpp

fn main() {
    fs::write("./tmp/a.txt", b"linux link").unwrap();
    let _ = std::os::unix::fs::symlink("./tmp/a.txt", "./tmp/a_link.txt").unwrap();
    let f = fs::read_link("./tmp/a_link.txt").unwrap();
    println!("{f:?}");
}
