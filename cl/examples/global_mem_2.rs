#![allow(non_camel_case_types, non_snake_case)]

use cl::ocl_v3::{
    gen_vector_program_source, CAPACITY_GLOBAL_ARRAY, EXTRACT_KERNEL_NAME, KERNEL_NAME, LIST_SIZE,
    TOTAL_GLOBAL_ARRAY,
};
use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
use opencl3::program::{Program, CL_STD_2_0};
use opencl3::types::{cl_event, cl_int, CL_BLOCKING};
use opencl3::Result;
use std::time::Duration;
use std::{ptr, thread};

fn main() -> Result<()> {
    let child = thread::Builder::new()
        .stack_size(30 * 1024 * 1024)
        .spawn(move || -> Result<()> {
            // const LIST_SIZE: usize = 1024 * 1024;
            // println!("{LIST_SIZE}");

            let mut A: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
            for i in 0..LIST_SIZE {
                A[i] = i as cl_int;
            }

            let device_id = *get_all_devices(CL_DEVICE_TYPE_GPU)?
                .first()
                .expect("no device found in platform");
            let device = Device::new(device_id);

            let context = Context::from_device(&device).expect("Context::from_device failed");
            println!("{context:?}");

            let queue = CommandQueue::create_default_with_properties(
                &context,
                CL_QUEUE_PROFILING_ENABLE,
                0,
            )
            .expect("CommandQueue::create_default failed");
            println!("{queue:?}");

            let mut a_mem_obj = unsafe {
                Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, LIST_SIZE, ptr::null_mut())?
            };
            let c_mem_obj = unsafe {
                Buffer::<cl_int>::create(&context, CL_MEM_WRITE_ONLY, LIST_SIZE, ptr::null_mut())?
            };

            let _a_write_event =
                unsafe { queue.enqueue_write_buffer(&mut a_mem_obj, CL_BLOCKING, 0, &A, &[])? };

            let program_source =
                gen_vector_program_source(TOTAL_GLOBAL_ARRAY, CAPACITY_GLOBAL_ARRAY);
            println!("{program_source}");
            let program = Program::create_and_build_from_source(
                &context,
                program_source.as_str(),
                CL_STD_2_0,
            )
            .expect("Program::create_and_build_from_source failed");
            println!("{program:?}");

            let kernel = Kernel::create(&program, KERNEL_NAME).expect("Kernel::create failed");
            println!("{kernel:?}");

            let extract_kernel =
                Kernel::create(&program, EXTRACT_KERNEL_NAME).expect("Kernel::create failed");
            println!("{extract_kernel:?}");

            // selected arr
            let D: [cl_int; LIST_SIZE] = [3; LIST_SIZE];

            let mut d_mem_obj = unsafe {
                Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, LIST_SIZE, ptr::null_mut())?
            };

            let _d_write_event =
                unsafe { queue.enqueue_write_buffer(&mut d_mem_obj, CL_BLOCKING, 0, &D, &[])? };

            let kernel_event = unsafe {
                ExecuteKernel::new(&kernel)
                    .set_arg(&a_mem_obj)
                    .set_arg(&d_mem_obj)
                    .set_global_work_size(LIST_SIZE)
                    .set_local_work_size(64)
                    .enqueue_nd_range(&queue)?
            };

            kernel_event.wait().expect("kernel_event.wait()");

            let kernel_event = unsafe {
                ExecuteKernel::new(&extract_kernel)
                    .set_arg(&c_mem_obj)
                    .set_arg(&d_mem_obj)
                    .set_global_work_size(LIST_SIZE)
                    .set_local_work_size(64)
                    .enqueue_nd_range(&queue)?
            };

            kernel_event.wait().expect("kernel_event.wait()");

            let mut events: Vec<cl_event> = Vec::default();
            events.push(kernel_event.get());

            let mut results: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
            let read_event = unsafe {
                queue.enqueue_read_buffer(&c_mem_obj, CL_BLOCKING, 0, &mut results, &events)?
            };

            // Wait for the read_event to complete.
            read_event.wait()?;

            for i in 0..5 {
                println!("{} -> {}", A[i], results[i]);
            }

            let last = LIST_SIZE - 5;
            for i in last..LIST_SIZE {
                println!("{} -> {}", A[i], results[i]);
            }

            println!("Thread #child finished!");
            thread::sleep(Duration::from_millis(2000));

            Ok(())
        })
        .unwrap();

    let _ = child.join().unwrap();

    println!("Thread #main finished!");
    Ok(())
}
