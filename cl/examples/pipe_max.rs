#![allow(non_camel_case_types, non_snake_case)]

use cl3::device::CL_DEVICE_TYPE_GPU;
use cl3::types::{cl_event, cl_int, CL_BLOCKING};
use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, Pipe, CL_MEM_READ_ONLY, CL_MEM_READ_WRITE, CL_MEM_WRITE_ONLY};
use opencl3::program::Program;
use opencl3::types::cl_uint;
use opencl3::Result;
use std::{mem, ptr};

const PROGRAM_SOURCE: &str = r#"
__kernel void vector_add(
    __global int* A, __global int* B, __global int* C,
    __write_only pipe int pipe1,
    __write_only pipe int pipe2,
    __write_only pipe int p0,
    __write_only pipe int p1,
    __write_only pipe int p2,
    __write_only pipe int p3,
    __write_only pipe int p4
    ) {

    // Get the index of the current element
    int i = get_global_id(0);

    // printf("global_id %d %d",  i, A[i]);
    // printf("A %d ", A[i]);
    // int v = A[i] + i;

    write_pipe(pipe1, &A[i]);
    write_pipe(pipe2, &i);

    switch (i) {
        case 0:
          write_pipe(p0, &A[i]);
          // printf("i0 %d", i);
          break;
        case 1:
          write_pipe(p1, &A[i]);
          // printf("i1 %d", i);
          break;
        case 2:
          write_pipe(p2, &A[i]);
          // printf("i2 %d", i);
          break;
        case 3:
          write_pipe(p3, &A[i]);
          // printf("i1 %d", i);
          break;
        case 4:
          write_pipe(p4, &A[i]);
          // printf("i2 %d", i);
          break;
        // default:
          // printf("default %d", i);
    }

    // Do the operation
    C[i] = A[i] + B[i];
}"#;

const KERNEL_NAME: &str = "vector_add";

const CONSUMER_PROGRAM_SOURCE: &str = r#"
__kernel void consumer(
    __global int* output, __global int* indices, __global int* values,
    __read_only pipe int pipe1, __read_only pipe int pipe2,
    __read_only pipe int p0,
    __read_only pipe int p1,
    __read_only pipe int p2,
    __read_only pipe int p3,
    __read_only pipe int p4
    ) {
    // Get the index of the current element
    int i = get_global_id(0);
    // printf("global_id %d",  i);

    //
    // for (int index = 0 ; index < 1024 ; index++) {
    //     read_pipe(pipe1, &output[index]);
    // }

    // int v;
    // char b[1] = "1";
    read_pipe(pipe1, &output[i]);
    read_pipe(pipe2, &indices[i]);
    // output[i] = v;

    values[i] = -1;

    switch (i) {
        case 0:
          read_pipe(p0, &values[i]);
          // printf("i1 %d", i);
          break;
        case 1:
          read_pipe(p1, &values[i]);
          // printf("i1 %d", i);
          break;
        case 2:
          read_pipe(p2, &values[i]);
          // printf("i2 %d", i);
          break;
        case 3:
          read_pipe(p3, &values[i]);
          // printf("i1 %d", i);
          break;
        case 4:
          read_pipe(p4, &values[i]);
          // printf("i2 %d", i);
          break;
         // default:
          // values[i] = -1;
          // break;
          // printf("default %d", i);
    }
}"#;

const CONSUMER_KERNEL_NAME: &str = "consumer";

fn main() -> Result<()> {
    const LIST_SIZE: usize = 1024;
    const TOTAL_PIPES: usize = 5;

    let mut A: [cl_int; LIST_SIZE] = [-1; LIST_SIZE];
    let mut B: [cl_int; LIST_SIZE] = [0; LIST_SIZE];

    // println!("{A:?}");

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

    // let consumer_queue =
    //     CommandQueue::create_default_with_properties(&context, CL_QUEUE_PROFILING_ENABLE, 0)
    //         .expect("CommandQueue::create_default failed");
    // println!("{consumer_queue:?}");

    // pipes
    let pipe = unsafe {
        Pipe::create(
            &context,
            CL_MEM_READ_WRITE,
            (LIST_SIZE * mem::size_of::<cl_int>()) as cl_uint,
            (LIST_SIZE * 10) as cl_uint,
        )
        .expect("Pipe::create failed")
    };
    println!("{pipe:?}");

    let pipe_index = unsafe {
        Pipe::create(
            &context,
            CL_MEM_READ_WRITE,
            (LIST_SIZE * mem::size_of::<cl_int>()) as cl_uint,
            (LIST_SIZE * 10) as cl_uint,
        )
        .expect("Pipe::create failed")
    };

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

    let program = Program::create_and_build_from_source(&context, PROGRAM_SOURCE, "-cl-std=CL2.0")
        // .expect("Program::create_and_build_from_source failed");
        .unwrap();
    println!("{program:?}");

    let kernel = Kernel::create(&program, KERNEL_NAME).expect("Kernel::create failed");
    println!("{kernel:?}");

    let consumer_program =
        Program::create_and_build_from_source(&context, CONSUMER_PROGRAM_SOURCE, "-cl-std=CL2.0")
            // .expect("Program::create_and_build_from_source failed");
            .unwrap();
    println!("{consumer_program:?}");

    let consumer_kernel = Kernel::create(&consumer_program, CONSUMER_KERNEL_NAME)
        .expect("Kernel::create failed CONSUMER_KERNEL_NAME");
    println!("{consumer_kernel:?}");

    let kernel_event = unsafe {
        let mut ex = ExecuteKernel::new(&kernel);

        ex.set_arg(&a_mem_obj)
            .set_arg(&b_mem_obj)
            .set_arg(&c_mem_obj)
            .set_arg(&pipe)
            .set_arg(&pipe_index);

        for p in pipe_vec.iter() {
            ex.set_arg(p);
        }

        ex.set_global_work_size(LIST_SIZE)
            .set_local_work_size(64)
            .enqueue_nd_range(&queue)?
    };

    // println!("{pipe_vec:?}");

    let mut events: Vec<cl_event> = Vec::default();
    events.push(kernel_event.get());

    let mut results: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
    let read_event =
        unsafe { queue.enqueue_read_buffer(&c_mem_obj, CL_BLOCKING, 0, &mut results, &events)? };

    // Wait for the read_event to complete.
    read_event.wait()?;

    let display = LIST_SIZE;

    // println!("kernel result");
    // for i in 0..display {
    //     println!("{} + {} = {}", A[i], B[i], results[i]);
    // }

    let d_mem_obj = unsafe {
        Buffer::<cl_int>::create(&context, CL_MEM_WRITE_ONLY, LIST_SIZE, ptr::null_mut())?
    };

    let e_mem_obj = unsafe {
        Buffer::<cl_int>::create(&context, CL_MEM_WRITE_ONLY, LIST_SIZE, ptr::null_mut())?
    };

    let kernel_event = unsafe {
        let mut ex = ExecuteKernel::new(&consumer_kernel);

        ex.set_arg(&c_mem_obj)
            .set_arg(&d_mem_obj)
            .set_arg(&e_mem_obj)
            .set_arg(&pipe)
            .set_arg(&pipe_index);

        for p in pipe_vec.iter() {
            ex.set_arg(p);
        }

        ex.set_global_work_size(LIST_SIZE)
            .set_local_work_size(64)
            .enqueue_nd_range(&queue)?
    };

    let mut events: Vec<cl_event> = Vec::default();
    events.push(kernel_event.get());

    let mut results: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
    let mut indices: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
    let mut values: [cl_int; LIST_SIZE] = [-1; LIST_SIZE];

    let read_event =
        unsafe { queue.enqueue_read_buffer(&c_mem_obj, CL_BLOCKING, 0, &mut results, &events)? };
    let read_event_ind =
        unsafe { queue.enqueue_read_buffer(&d_mem_obj, CL_BLOCKING, 0, &mut indices, &events)? };
    let read_event_values =
        unsafe { queue.enqueue_read_buffer(&e_mem_obj, CL_BLOCKING, 0, &mut values, &events)? };

    // Wait for the read_event to complete.
    read_event.wait()?;
    read_event_ind.wait()?;
    read_event_values.wait()?;

    // println!("kernel pipe indices");
    // for i in 0..display {
    //     println!("{} + {} = {}", A[i], B[i], indices[i]);
    // }

    // println!("{results:?}");

    // println!("kernel pipe result");
    // for i in 0..display {
    //     println!("{} + {} = v{} -> i{} ", A[i], B[i], results[i], indices[i]);
    // }

    println!("kernel pipe values");
    for i in 0..display {
        println!("{} + {} = {}", A[i], B[i], values[i]);
    }

    // let mut vec: Vec<(&i32, &i32)> = vec![];
    //
    // for (x, y) in results.iter().zip(indices.iter()) {
    //     println!("x{x} y{y}");
    //     vec.push((x, y));
    // }
    //
    // vec.sort_by(|a, b| a.1.cmp(b.1));
    //
    // for v  in vec {
    //     println!("x{} y{}", v.0, v.1);
    // }

    Ok(())
}
