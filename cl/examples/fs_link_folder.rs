use std::fs;

// ln -s /codes/v.cpp /codes/link.cpp

// ln -s /codes/folder_test /codes/folder_link

// ln -s ./a.txt ./link_a.txt
// ln -s ./test_dir ./link_test_dir

fn main() {
    fs::create_dir_all("./tmp/link_dir").unwrap();

    fs::write("./tmp/link_dir/a.txt", b"linux link").unwrap();
    // let _ = std::os::unix::fs::symlink("./tmp/link_dir", "./tmp/ref_link_dir").unwrap();
    let f = fs::read_link("./tmp/ref_link_dir").unwrap();
    println!("{f:?}");
}
