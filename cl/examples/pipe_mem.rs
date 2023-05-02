use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, Pipe, CL_MEM_READ_ONLY, CL_MEM_READ_WRITE, CL_MEM_WRITE_ONLY};
use opencl3::program::{Program, CL_STD_2_0};
use opencl3::types::{cl_event, cl_int, cl_uint, CL_BLOCKING};
use opencl3::Result;
use std::{mem, ptr};

const PRODUCER_PROGRAM_SOURCE: &str = r#"
__kernel void producer(
    __global int* input,
    __write_only pipe int p0,
    __write_only pipe int p1,
    __write_only pipe int p2,
    __write_only pipe int p3,
    __write_only pipe int p4
    ) {

    // Get the index of the current element
    int i = get_global_id(0);
    // printf("global_id %d %d",  i, input[i]);

    switch (i) {
        case 0:
          write_pipe(p0, &input[i]);
          break;
        case 1:
          write_pipe(p1, &input[i]);
          break;
        case 2:
          write_pipe(p2, &input[i]);
          break;
        case 3:
          write_pipe(p3, &input[i]);
          break;
        case 4:
          write_pipe(p4, &input[i]);
          break;
    }
}"#;

const PRODUCER_KERNEL_NAME: &str = "producer";

const CONSUMER_PROGRAM_SOURCE: &str = r#"
__kernel void consumer(
    __global int* output,
    __read_only pipe int p0,
    __read_only pipe int p1,
    __read_only pipe int p2,
    __read_only pipe int p3,
    __read_only pipe int p4
    ) {

    // Get the index of the current element
    int i = get_global_id(0);
    // printf("global_id %d",  i);

    output[i] = -1;

    switch (i) {
        case 0:
          read_pipe(p0, &output[i]);
          break;
        case 1:
          read_pipe(p1, &output[i]);
          break;
        case 2:
          read_pipe(p2, &output[i]);
          break;
        case 3:
          read_pipe(p3, &output[i]);
          break;
        case 4:
          read_pipe(p4, &output[i]);
          break;
    }
}"#;

const CONSUMER_KERNEL_NAME: &str = "consumer";

const LIST_SIZE: usize = 5;
const TOTAL_PIPES: usize = 5;

fn main() -> Result<()> {
    let h = String::from("hello");
    let h_b = h.as_bytes();
    println!("input: {h} -> {h_b:?}");

    let mut input: [cl_int; LIST_SIZE] = [-1; LIST_SIZE];

    for i in 0..LIST_SIZE {
        input[i] = h_b[i] as cl_int;
    }

    println!("input arr: {input:?}");

    let device_id = *get_all_devices(CL_DEVICE_TYPE_GPU)?
        .first()
        .expect("no device found in platform");
    let device = Device::new(device_id);
    println!("{device:?}");

    let context = Context::from_device(&device).expect("Context::from_device failed");
    println!("{context:?}");

    let queue =
        CommandQueue::create_default_with_properties(&context, CL_QUEUE_PROFILING_ENABLE, 0)
            .expect("CommandQueue::create_default failed");
    println!("{queue:?}");

    let mut pipe_vec = Vec::with_capacity(TOTAL_PIPES);
    for _ in 0..TOTAL_PIPES {
        pipe_vec.push(unsafe {
            Pipe::create(
                &context,
                CL_MEM_READ_WRITE,
                (LIST_SIZE * mem::size_of::<cl_int>()) as cl_uint,
                (LIST_SIZE * 10) as cl_uint,
            )
            .expect("Pipe::create failed")
        })
    }
    println!("{pipe_vec:?}");

    // buffers
    let mut input_mem_obj = unsafe {
        Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, LIST_SIZE, ptr::null_mut())?
    };

    let output_mem_obj = unsafe {
        Buffer::<cl_int>::create(&context, CL_MEM_WRITE_ONLY, LIST_SIZE, ptr::null_mut())?
    };

    let _a_write_event =
        unsafe { queue.enqueue_write_buffer(&mut input_mem_obj, CL_BLOCKING, 0, &input, &[])? };

    let producer_program =
        Program::create_and_build_from_source(&context, PRODUCER_PROGRAM_SOURCE, CL_STD_2_0)
            .expect("Program::create_and_build_from_source failed");
    println!("{producer_program:?}");

    let producer_kernel =
        Kernel::create(&producer_program, PRODUCER_KERNEL_NAME).expect("Kernel::create failed");
    println!("{producer_kernel:?}");

    let consumer_program =
        Program::create_and_build_from_source(&context, CONSUMER_PROGRAM_SOURCE, "-cl-std=CL2.0")
            .expect("Program::create_and_build_from_source failed");
    println!("{consumer_program:?}");

    let consumer_kernel =
        Kernel::create(&consumer_program, CONSUMER_KERNEL_NAME).expect("Kernel::create failed");
    println!("{consumer_kernel:?}");

    // producer

    let _ = unsafe {
        let mut ex = ExecuteKernel::new(&producer_kernel);

        ex.set_arg(&input_mem_obj);

        for p in pipe_vec.iter() {
            ex.set_arg(p);
        }

        ex.set_global_work_size(LIST_SIZE)
            .set_local_work_size(64)
            .enqueue_nd_range(&queue)?
    };

    // println!("{pipe_vec:?}");

    // consumer

    let kernel_event = unsafe {
        let mut ex = ExecuteKernel::new(&consumer_kernel);

        ex.set_arg(&output_mem_obj);

        for p in pipe_vec.iter() {
            ex.set_arg(p);
        }

        ex.set_global_work_size(LIST_SIZE)
            .set_local_work_size(64)
            .enqueue_nd_range(&queue)?
    };

    let mut events: Vec<cl_event> = Vec::default();
    events.push(kernel_event.get());

    let mut output: [cl_int; LIST_SIZE] = [-1; LIST_SIZE];

    let read_event = unsafe {
        queue.enqueue_read_buffer(&output_mem_obj, CL_BLOCKING, 0, &mut output, &events)?
    };

    // Wait for the read_event to complete.
    read_event.wait()?;

    let display = LIST_SIZE;

    println!("kernel pipe output");
    for i in 0..display {
        println!("i{} + v{}", i, output[i]);
    }

    Ok(())
}
