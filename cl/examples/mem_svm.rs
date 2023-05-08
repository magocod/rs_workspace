#![allow(non_camel_case_types, non_snake_case)]

use cl::hello;
use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, CL_MAP_WRITE, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
use opencl3::program::{Program, CL_STD_2_0};
use opencl3::svm::SvmVec;
use opencl3::types::{cl_int, CL_BLOCKING};
use opencl3::Result;
use std::ptr;

const PROGRAM_SOURCE: &str = r#"
kernel void vector_add(global int* A, global int* B, global int* C) {

    // Get the index of the current element
    int i = get_global_id(0);
    size_t lid = get_local_id(0);
    size_t lsize = get_local_size(0);

    // printf("global_id %d %d",  i, lid);

    size_t lidx = i * lsize + lid;
    C[lidx] = 10;

    // Do the operation
    // C[i] = A[i] + B[i];
    // C[i] = 10;
}"#;

const KERNEL_NAME: &str = "vector_add";

fn main() -> Result<()> {
    println!("{}", hello());

    const LIST_SIZE: usize = 1024;

    let mut A: [cl_int; LIST_SIZE] = [1; LIST_SIZE];
    let mut B: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
    for i in 0..LIST_SIZE {
        A[i] = i as cl_int;
        B[i] = (LIST_SIZE - i) as cl_int;
        // B[i] = i as cl_int;
    }

    let device_id = *get_all_devices(CL_DEVICE_TYPE_GPU)?
        .first()
        .expect("no device found in platform");
    let device = Device::new(device_id);

    let context = Context::from_device(&device).expect("Context::from_device failed");
    println!("{context:?}");

    let queue =
        CommandQueue::create_default_with_properties(&context, CL_QUEUE_PROFILING_ENABLE, 0)
            .expect("CommandQueue::create_default failed");
    println!("{queue:?}");

    // let mut a_mem_obj = unsafe {
    //     Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, LIST_SIZE, ptr::null_mut())?
    // };
    // let mut b_mem_obj = unsafe {
    //     Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, LIST_SIZE, ptr::null_mut())?
    // };
    // let c_mem_obj = unsafe {
    //     Buffer::<cl_int>::create(&context, CL_MEM_WRITE_ONLY, LIST_SIZE, ptr::null_mut())?
    // };

    // let _a_write_event =
    //     unsafe { queue.enqueue_write_buffer(&mut a_mem_obj, CL_BLOCKING, 0, &A, &[])? };
    // let _b_write_event =
    //     unsafe { queue.enqueue_write_buffer(&mut b_mem_obj, CL_BLOCKING, 0, &B, &[])? };

    // Create an OpenCL SVM vector
    let mut a_mem_obj =
        SvmVec::<cl_int>::allocate(&context, LIST_SIZE).expect("SVM allocation failed");
    // println!("a_mem_obj.is_fine_grained(), {}", a_mem_obj.is_fine_grained());

    // Copy input data into the OpenCL SVM vector
    a_mem_obj.clone_from_slice(&A);
    // println!("a_mem_obj.is_fine_grained(), {}", a_mem_obj.is_fine_grained());

    if !a_mem_obj.is_fine_grained() {
        unsafe { queue.enqueue_svm_map(CL_BLOCKING, CL_MAP_WRITE, &mut a_mem_obj, &[])? };
    }

    // Make test_values immutable
    let a_mem_obj = a_mem_obj;

    // Create an OpenCL SVM vector
    let mut b_mem_obj =
        SvmVec::<cl_int>::allocate(&context, LIST_SIZE).expect("SVM allocation failed");

    // Copy input data into the OpenCL SVM vector
    b_mem_obj.clone_from_slice(&B);

    if !b_mem_obj.is_fine_grained() {
        unsafe { queue.enqueue_svm_map(CL_BLOCKING, CL_MAP_WRITE, &mut b_mem_obj, &[])? };
    }

    // Make test_values immutable
    let b_mem_obj = b_mem_obj;

    // The output data, an OpenCL SVM vector
    let mut c_mem_obj =
        SvmVec::<cl_int>::allocate(&context, LIST_SIZE).expect("SVM allocation failed");

    let program = Program::create_and_build_from_source(&context, PROGRAM_SOURCE, CL_STD_2_0)
        .expect("Program::create_and_build_from_source failed");
    println!("{program:?}");

    let kernel = Kernel::create(&program, KERNEL_NAME).expect("Kernel::create failed");
    println!("{kernel:?}");

    let kernel_event = unsafe {
        ExecuteKernel::new(&kernel)
            .set_arg_svm(&a_mem_obj.as_ptr())
            .set_arg_svm(&b_mem_obj.as_ptr())
            .set_arg_svm(&c_mem_obj.as_mut_ptr())
            .set_global_work_size(LIST_SIZE)
            .set_local_work_size(64)
            .enqueue_nd_range(&queue)?
    };

    // Wait for the kernel to complete execution on the device
    kernel_event.wait()?;

    // let mut events: Vec<cl_event> = Vec::default();
    // events.push(kernel_event.get());
    //
    // let mut results: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
    // let read_event =
    //     unsafe { queue.enqueue_read_buffer(&c_mem_obj, CL_BLOCKING, 0, &mut results, &events)? };
    //
    // // Wait for the read_event to complete.
    // read_event.wait()?;

    for i in 0..LIST_SIZE {
        println!("{} + {} = {}", A[i], B[i], c_mem_obj[i]);
    }

    Ok(())
}
