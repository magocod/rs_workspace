#![allow(non_camel_case_types, non_snake_case)]

use cl::hello;
use cl3::device::CL_DEVICE_TYPE_GPU;
use cl3::types::{cl_event, cl_int, CL_BLOCKING};
use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
use opencl3::program::{Program, CL_STD_2_0};
use opencl3::Result;
use std::time::Duration;
use std::{ptr, thread};

const PROGRAM_SOURCE: &str = r#"
kernel void vector_add(global int* A, global int* B, global int* C) {

    // Get the index of the current element
    int i = get_global_id(0);

    // Do the operation
    C[i] = A[i] + B[i];
}"#;

const KERNEL_NAME: &str = "vector_add";

const GLOBAL_PROGRAM_SOURCE: &str = r#"
// __constant int kb_1 = 1024;
// __constant int kb_n = 20 * kb_1 * kb_1; // 20971520

__global int globalA;
// __global int* globalB;
__global int myNumbers[20971520];

struct myStructure {
    int myNum;
    char myLetter;
};

__global struct myStructure myStructures[6];

kernel void update_global() {
    // Get the index of the current element
    int i = get_global_id(0);

    globalA = 75;
    myNumbers[i] = 76;

    struct myStructure s2 = { 14, 'C' };
    myStructures[i] = s2;

    printf("globalA %d", globalA);
    printf("myNumbers[i] %d %d", myNumbers[i], i);

    size_t sizeMyNumbers = sizeof(myNumbers);
    printf("myNumbers size %d \n", sizeMyNumbers);

    printf("myStructures[].myNum: %d\n", myStructures[i].myNum);
    printf("myStructures[].myLetter: %c\n", myStructures[i].myLetter);
}

kernel void get_update_global() {
    // Get the index of the current element
    int i = get_global_id(0);

    printf("get_globalA %d", globalA);
    printf("myNumbers[i] %d %d", myNumbers[i], i);

    printf("myStructures[].myNum: %d\n", myStructures[i].myNum);
    printf("myStructures[].myLetter: %c\n", myStructures[i].myLetter);
}
"#;

const GLOBAL_KERNEL_NAME: &str = "update_global";

// const GET_GLOBAL_PROGRAM_SOURCE: &str = r#"
// kernel void get_update_global() {
//     printf("get_globalA %d", globalA);
// }"#;

const GET_GLOBAL_KERNEL_NAME: &str = "get_update_global";

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

    let global_program =
        Program::create_and_build_from_source(&context, GLOBAL_PROGRAM_SOURCE, CL_STD_2_0)
            .expect("global_program::create_and_build_from_source failed");
    println!("{global_program:?}");

    let global_kernel =
        Kernel::create(&global_program, GLOBAL_KERNEL_NAME).expect("Kernel::create failed");
    println!("{global_kernel:?}");

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

    let mut results: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
    let read_event =
        unsafe { queue.enqueue_read_buffer(&c_mem_obj, CL_BLOCKING, 0, &mut results, &events)? };

    // Wait for the read_event to complete.
    read_event.wait()?;

    for i in 0..LIST_SIZE {
        println!("{} + {} = {}", A[i], B[i], results[i]);
    }

    let global_kernel_event = unsafe {
        ExecuteKernel::new(&global_kernel)
            .set_global_work_size(1)
            .set_local_work_size(1)
            .enqueue_nd_range(&queue)?
    };

    global_kernel_event.wait().expect("global_kernel_event");

    let get_global_kernel = Kernel::create(&global_program, GET_GLOBAL_KERNEL_NAME)
        .expect("get_global_kernel::create failed");
    println!("{get_global_kernel:?}");

    let get_global_kernel_event = unsafe {
        ExecuteKernel::new(&get_global_kernel)
            .set_global_work_size(1)
            .set_local_work_size(1)
            .enqueue_nd_range(&queue)?
    };

    get_global_kernel_event
        .wait()
        .expect("get_global_kernel_event");

    thread::sleep(Duration::from_millis(2000));

    Ok(())
}
