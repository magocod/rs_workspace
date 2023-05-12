#![allow(dead_code)]

use cl::ocl_fs::{ocl_cache, ocl_initialize, OclFile};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::{env};

fn generate_path(p: &str) -> String {
    let mut path = env::current_dir().unwrap();
    path.push(p);
    path.to_str().unwrap().to_string()
}

fn prepare_opencl(path: &str) {
    let f = File::open(path).unwrap();
    let mut reader = BufReader::new(f);
    let buffer = reader.fill_buf().unwrap();

    let mut f = OclFile::create(path).unwrap();
    f.write(buffer).unwrap();
}

fn fs_open_benchmark(c: &mut Criterion) {
    let path = generate_path("resources/to_open.js");
    prepare_opencl(&path);

    c.bench_function("File::open", |b| {
        b.iter(|| File::open(black_box(&path)))
    });
    c.bench_function("OclFile::open", |b| {
        b.iter(|| OclFile::open(black_box(&path)))
    });

    ocl_cache().unwrap();
}

fn fs_create_benchmark(c: &mut Criterion) {
    let path = generate_path("resources/to_create.txt");
    ocl_initialize(true);

    c.bench_function("File::create", |b| {
        b.iter(|| File::create(black_box(&path)))
    });
    c.bench_function("OclFile::create", |b| {
        b.iter(|| OclFile::create(black_box(&path)))
    });

    // fs::remove_file(path).unwrap();
    ocl_cache().unwrap();
}

fn fs_read(path: &str) {
    let f = File::open(path).unwrap();
    let mut reader = BufReader::new(f);
    let _ = reader.fill_buf().unwrap();
}

fn ocl_fs_read(path: &str) {
    let f = OclFile::open(path).unwrap();
    let mut reader = BufReader::new(f);
    let _ = reader.fill_buf().unwrap();
}

fn fs_read_benchmark(c: &mut Criterion) {
    let path = generate_path("resources/to_read.js");
    prepare_opencl(&path);

    c.bench_function("File::read", |b| {
        b.iter(|| fs_read(black_box(&path)))
    });
    c.bench_function("OclFile::read", |b| {
        b.iter(|| ocl_fs_read(black_box(&path)))
    });

    ocl_cache().unwrap();
}

fn fs_write(path: &str, data: &[u8]) {
    let mut f = File::create(path).unwrap();
    f.write(data).unwrap();
}

fn ocl_fs_write(path: &str, data: &[u8]) {
    let mut f = OclFile::create(path).unwrap();
    f.write(data).unwrap();
}

fn fs_write_benchmark(c: &mut Criterion) {
    let value = b"value";
    let path = generate_path("resources/to_write.txt");
    ocl_initialize(true);

    c.bench_function("File::write", |b| {
        b.iter(|| fs_write(black_box(&path), black_box(value)))
    });
    c.bench_function("OclFile::write", |b| {
        b.iter(|| ocl_fs_write(black_box(&path), black_box(value)))
    });

    ocl_cache().unwrap();
}

criterion_group!(
    benches,
    fs_open_benchmark,
    fs_create_benchmark,
    fs_read_benchmark,
    fs_write_benchmark
);
criterion_main!(benches);
