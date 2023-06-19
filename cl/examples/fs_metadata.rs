use std::fs;


fn main() {
    let m = fs::metadata("./cl/examples/files/info.txt").unwrap();
    let file_type = m.file_type();

    println!("{file_type:?}");
    println!("{m:?}");

    let f = fs::read_link("./tmp/link_a.txt").unwrap();
    println!("{f:?}");

    // let m = fs::metadata("./tmp/a_link.txt").unwrap(); // error
    let m = fs::symlink_metadata("./tmp/link_a.txt").unwrap();
    println!("{m:?}");

    let m = fs::symlink_metadata("./tmp/link_test_dir").unwrap();
    println!("{m:?}");
}
