use cl::ocl_fs_2;
use cl::ocl_fs_2::OclFile;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let b = b"Hello, World!";
    let path_create = "./cl/examples/output/data.txt";
    let path_fs_create = "./cl/examples/output/fs_write.txt";
    let path_fs_delete = "./cl/examples/output/del.txt";

    // Create a file
    let mut data_file = File::create(path_create).expect("creation failed");

    // Write contents to the file
    data_file.write(b).expect("write failed");

    File::create(path_fs_delete).expect("creation failed");
    fs::remove_file(path_fs_delete).unwrap();

    fs::write(path_fs_create, b).unwrap();

    // opencl
    let mut f = OclFile::create(path_create).unwrap();
    f.write(b).unwrap();

    ocl_fs_2::write(path_fs_create, b).unwrap();
    ocl_fs_2::write(path_fs_create, "update").unwrap();

    ocl_fs_2::ocl_cache().unwrap();
}
