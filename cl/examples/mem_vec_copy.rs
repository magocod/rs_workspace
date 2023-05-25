#![allow(non_camel_case_types, non_snake_case)]

use cl::hello;
use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{Device, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
use opencl3::program::{Program, CL_STD_2_0};
use opencl3::types::{cl_event, cl_int, CL_BLOCKING};
use opencl3::Result;
use std::ptr;

const PROGRAM_SOURCE: &str = r#"
__global int arrA[1024];
__global int arrB[1024];
__global int arrC[1024];

int* get_arr(const int index) {
    switch (index) {
        case 0:
          return arrA;
        case 1:
          return arrB;
        case 2:
          return arrC;
    }
}

kernel void vector_add(global int* A, global int* B, global int* C) {

    // Get the index of the current element
    int i = get_global_id(0);

    // Do the operation
    C[i] = A[i] + B[i];
    arrA[i] = 10;
    arrC[i] = 11;
}

kernel void vector_extract(global int* output, const int index) {

    // Get the index of the current element
    int i = get_global_id(0);

    int* arr = get_arr(index);

    // Do the operation
    output[i] = arr[i];
}

kernel void copy_to(const int from, const int to) {

    // Get the index of the current element
    int i = get_global_id(0);

    // Do the operation
    int* from_arr = get_arr(from);
    int* to_arr = get_arr(to);

    to_arr[i] = from_arr[i];
}

kernel void get_arrA(global int* A) {

    // Get the index of the current element
    int i = get_global_id(0);

    // Do the operation
    A[i] = arrA[i];
}

kernel void get_arrB(global int* B) {

    // Get the index of the current element
    int i = get_global_id(0);

    // Do the operation
    B[i] = arrB[i];
}

"#;

const KERNEL_NAME: &str = "vector_add";

fn main() -> Result<()> {
    println!("{}", hello());

    const LIST_SIZE: usize = 1024;

    // let mut A: [cl_int; LIST_SIZE] = [1; LIST_SIZE];
    let mut A = vec![1; LIST_SIZE];
    // let mut B: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
    let mut B = vec![0; LIST_SIZE];
    for i in 0..LIST_SIZE {
        A[i] = i as cl_int;
        B[i] = (LIST_SIZE - i) as cl_int;
        // B[i] = i as cl_int;
    }

    // Find a usable platform and device for this application
    let platforms = opencl3::platform::get_platforms()?;
    let platform = platforms.first().expect("no OpenCL platforms");

    // let device_id = *get_all_devices(CL_DEVICE_TYPE_GPU)?
    //     .first()
    //     .expect("no device found in platform");
    let device_id = *platform
        .get_devices(CL_DEVICE_TYPE_GPU)?
        .first()
        .expect("no device found in platform");

    let device = Device::new(device_id);

    let context = Context::from_device(&device).expect("Context::from_device failed");
    println!("{context:?}");

    let queue =
        CommandQueue::create_default_with_properties(&context, CL_QUEUE_PROFILING_ENABLE, 0)
            .expect("CommandQueue::create_default failed");
    println!("{queue:?}");

    let mut a_mem_obj = unsafe {
        Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, LIST_SIZE, ptr::null_mut())?
    };
    let mut b_mem_obj = unsafe {
        Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, LIST_SIZE, ptr::null_mut())?
    };
    let c_mem_obj = unsafe {
        Buffer::<cl_int>::create(&context, CL_MEM_WRITE_ONLY, LIST_SIZE, ptr::null_mut())?
    };

    let _a_write_event =
        unsafe { queue.enqueue_write_buffer(&mut a_mem_obj, CL_BLOCKING, 0, &A, &[])? };
    let _b_write_event =
        unsafe { queue.enqueue_write_buffer(&mut b_mem_obj, CL_BLOCKING, 0, &B, &[])? };

    let program = Program::create_and_build_from_source(&context, PROGRAM_SOURCE, CL_STD_2_0)
        .expect("Program::create_and_build_from_source failed");
    println!("{program:?}");

    let kernel = Kernel::create(&program, KERNEL_NAME).expect("Kernel::create failed");
    println!("{kernel:?}");

    let kernel_event = unsafe {
        ExecuteKernel::new(&kernel)
            .set_arg(&a_mem_obj)
            .set_arg(&b_mem_obj)
            .set_arg(&c_mem_obj)
            .set_global_work_size(LIST_SIZE)
            .set_local_work_size(64)
            .enqueue_nd_range(&queue)?
    };

    let mut events: Vec<cl_event> = Vec::default();
    events.push(kernel_event.get());

    // let mut results: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
    let mut results = vec![0; LIST_SIZE];

    let read_event =
        unsafe { queue.enqueue_read_buffer(&c_mem_obj, CL_BLOCKING, 0, &mut results, &events)? };

    // Wait for the read_event to complete.
    read_event.wait()?;

    // for i in 0..LIST_SIZE {
    //     println!("{} + {} = {}", A[i], B[i], results[i]);
    // }

    // read arr a
    let kernel = Kernel::create(&program, "get_arrA").expect("Kernel::create failed");
    println!("{kernel:?}");

    let kernel_event = unsafe {
        ExecuteKernel::new(&kernel)
            .set_arg(&c_mem_obj)
            .set_global_work_size(LIST_SIZE)
            .set_local_work_size(64)
            .enqueue_nd_range(&queue)?
    };

    let mut events: Vec<cl_event> = Vec::default();
    events.push(kernel_event.get());

    // let mut results: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
    let mut results = vec![0; LIST_SIZE];

    let read_event =
        unsafe { queue.enqueue_read_buffer(&c_mem_obj, CL_BLOCKING, 0, &mut results, &events)? };

    // Wait for the read_event to complete.
    read_event.wait()?;

    println!("arrA");
    for i in 0..10 {
        println!("{} = {}", i, results[i]);
    }

    let kernel = Kernel::create(&program, "copy_to").expect("Kernel::create failed");
    println!("{kernel:?}");

    let a_index = 0;
    let b_index = 1;

    unsafe {
        ExecuteKernel::new(&kernel)
            .set_arg(&a_index)
            .set_arg(&b_index)
            .set_global_work_size(LIST_SIZE)
            .set_local_work_size(64)
            .enqueue_nd_range(&queue)?
    };

    // read arr b
    let kernel = Kernel::create(&program, "get_arrB").expect("Kernel::create failed");
    println!("{kernel:?}");

    let kernel_event = unsafe {
        ExecuteKernel::new(&kernel)
            .set_arg(&c_mem_obj)
            .set_global_work_size(LIST_SIZE)
            .set_local_work_size(64)
            .enqueue_nd_range(&queue)?
    };

    let mut events: Vec<cl_event> = Vec::default();
    events.push(kernel_event.get());

    // let mut results: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
    let mut results = vec![0; LIST_SIZE];

    let read_event =
        unsafe { queue.enqueue_read_buffer(&c_mem_obj, CL_BLOCKING, 0, &mut results, &events)? };

    // Wait for the read_event to complete.
    read_event.wait()?;

    println!("arrB");
    for i in 0..10 {
        println!("{} = {}", i, results[i]);
    }

    // select and read arr b
    let kernel = Kernel::create(&program, "vector_extract").expect("Kernel::create failed");
    println!("{kernel:?}");

    let c_index = 2;

    let kernel_event = unsafe {
        ExecuteKernel::new(&kernel)
            .set_arg(&c_mem_obj)
            .set_arg(&c_index)
            .set_global_work_size(LIST_SIZE)
            .set_local_work_size(64)
            .enqueue_nd_range(&queue)?
    };

    let mut events: Vec<cl_event> = Vec::default();
    events.push(kernel_event.get());

    // let mut results: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
    let mut results = vec![0; LIST_SIZE];

    let read_event =
        unsafe { queue.enqueue_read_buffer(&c_mem_obj, CL_BLOCKING, 0, &mut results, &events)? };

    // Wait for the read_event to complete.
    read_event.wait()?;

    println!("select arrC");
    for i in 0..10 {
        println!("{} = {}", i, results[i]);
    }

    Ok(())
}
