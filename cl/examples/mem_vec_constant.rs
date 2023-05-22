#![allow(non_camel_case_types, non_snake_case)]

use cl::hello;
use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
use opencl3::program::Program;
use opencl3::types::{cl_event, cl_int, CL_BLOCKING};
use opencl3::Result;
use std::ptr;

const PROGRAM_SOURCE: &str = r#"
kernel void vector_add(const int A, const int B, global int* C) {

    // Get the index of the current element
    int i = get_global_id(0);

    // Do the operation
    C[i] = A + B;
}"#;

const KERNEL_NAME: &str = "vector_add";

fn main() -> Result<()> {
    println!("{}", hello());

    const LIST_SIZE: usize = 1024;

    let A = 1;
    let B = 2;

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


    let c_mem_obj = unsafe {
        Buffer::<cl_int>::create(&context, CL_MEM_WRITE_ONLY, LIST_SIZE, ptr::null_mut())?
    };


    let program = Program::create_and_build_from_source(&context, PROGRAM_SOURCE, "")
        .expect("Program::create_and_build_from_source failed");
    println!("{program:?}");

    let kernel = Kernel::create(&program, KERNEL_NAME).expect("Kernel::create failed");
    println!("{kernel:?}");

    let kernel_event = unsafe {
        ExecuteKernel::new(&kernel)
            .set_arg(&A)
            .set_arg(&B)
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

    for i in 0..LIST_SIZE {
        println!("{} + {} = {}", A, B, results[i]);
    }

    Ok(())
}
