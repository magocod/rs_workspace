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

// const PROGRAM_SOURCE: &str = r#"
// kernel void saxpy_float (global float* z,
//     global float const* x,
//     global float const* y,
//     float a)
// {
//     const size_t i = get_global_id(0);
//     z[i] = a*x[i] + y[i];
// }"#;

// const PROGRAM_SOURCE: &str = r#"
// __kernel void vector_add(__global int *A, __global int *B, __global int *C) {
//
//     // Get the index of the current element
//     int i = get_global_id(0);
//
//     // Do the operation
//     C[i] = A[i] + B[i];
// }"#;

const PROGRAM_SOURCE: &str = r#"
kernel void vector_add(global int* A, global int* B, global int* C) {

    // Get the index of the current element
    int i = get_global_id(0);

    // Do the operation
    C[i] = A[i] + B[i];
}"#;

const KERNEL_NAME: &str = "vector_add";

fn main() -> Result<()> {
    println!("{}", hello());

    // let platforms = get_platform_ids()?;
    // println!("Number of platforms: {}", platforms.len());
    //
    // let platform_id = platforms.first().unwrap();
    // println!("platform_id: {platform_id:#?}");
    //
    // let devices = get_device_ids(platform_id.clone(), CL_DEVICE_TYPE_ALL)?;
    // println!("Number of devices: {}", devices.len());
    //
    // let device_id = devices.first().unwrap();
    // println!("device_id: {device_id:#?}");

    // Create the two input vectors
    // int i;
    // const int LIST_SIZE = 1024;
    // int *A = (int*)malloc(sizeof(int)*LIST_SIZE);
    // int *B = (int*)malloc(sizeof(int)*LIST_SIZE);
    // for(i = 0; i < LIST_SIZE; i++) {
    //     A[i] = i;
    //     B[i] = LIST_SIZE - i;
    // }

    const LIST_SIZE: usize = 1024;

    let mut A: [cl_int; LIST_SIZE] = [1; LIST_SIZE];
    let mut B: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
    for i in 0..LIST_SIZE {
        A[i] = i as cl_int;
        B[i] = (LIST_SIZE - i) as cl_int;
        // B[i] = i as cl_int;
    }

    // let mut A: Vec<usize> = Vec::new();
    // let mut B: Vec<usize> = Vec::new();
    //
    // for i in 0..LIST_SIZE {
    //     A.push(i);
    //     B.push(LIST_SIZE - i);
    // }

    // println!("{A:?}");
    // println!("{B:?}");

    // // Load the kernel source code into the array source_str
    // FILE *fp;
    // char *source_str;
    // size_t source_size;
    //
    // fp = fopen("vector_add_kernel.cl", "r");
    // if (!fp) {
    //     fprintf(stderr, "Failed to load kernel.\n");
    //     exit(1);
    // }
    // source_str = (char*)malloc(MAX_SOURCE_SIZE);
    // source_size = fread( source_str, 1, MAX_SOURCE_SIZE, fp);
    // fclose( fp );
    //
    // // Get platform and device information
    // cl_platform_id platform_id = NULL;
    // cl_device_id device_id = NULL;
    // cl_uint ret_num_devices;
    // cl_uint ret_num_platforms;
    // cl_int ret = clGetPlatformIDs(1, &platform_id, &ret_num_platforms);
    // ret = clGetDeviceIDs( platform_id, CL_DEVICE_TYPE_DEFAULT, 1,
    //                       &device_id, &ret_num_devices);
    //
    // // Create an OpenCL context
    // cl_context context = clCreateContext( NULL, 1, &device_id, NULL, NULL, &ret);

    let device_id = *get_all_devices(CL_DEVICE_TYPE_GPU)?
        .first()
        .expect("no device found in platform");
    let device = Device::new(device_id);

    let context = Context::from_device(&device).expect("Context::from_device failed");
    println!("{context:?}");

    // // Create a command queue
    // cl_command_queue command_queue = clCreateCommandQueue(context, device_id, 0, &ret);

    // let queue = CommandQueue::create_default(&context, CL_QUEUE_PROFILING_ENABLE)
    let queue =
        CommandQueue::create_default_with_properties(&context, CL_QUEUE_PROFILING_ENABLE, 0)
            .expect("CommandQueue::create_default failed");
    println!("{queue:?}");

    // // Create memory buffers on the device for each vector
    // cl_mem a_mem_obj = clCreateBuffer(context, CL_MEM_READ_ONLY,
    //                                   LIST_SIZE * sizeof(int), NULL, &ret);
    // cl_mem b_mem_obj = clCreateBuffer(context, CL_MEM_READ_ONLY,
    //                                   LIST_SIZE * sizeof(int), NULL, &ret);
    // cl_mem c_mem_obj = clCreateBuffer(context, CL_MEM_WRITE_ONLY,

    let mut a_mem_obj = unsafe {
        Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, LIST_SIZE, ptr::null_mut())?
    };
    let mut b_mem_obj = unsafe {
        Buffer::<cl_int>::create(&context, CL_MEM_READ_ONLY, LIST_SIZE, ptr::null_mut())?
    };
    let c_mem_obj = unsafe {
        Buffer::<cl_int>::create(&context, CL_MEM_WRITE_ONLY, LIST_SIZE, ptr::null_mut())?
    };

    // // Copy the lists A and B to their respective memory buffers
    // ret = clEnqueueWriteBuffer(command_queue, a_mem_obj, CL_TRUE, 0,
    //                            LIST_SIZE * sizeof(int), A, 0, NULL, NULL);
    // ret = clEnqueueWriteBuffer(command_queue, b_mem_obj, CL_TRUE, 0,
    //                            LIST_SIZE * sizeof(int), B, 0, NULL, NULL);

    let _a_write_event =
        unsafe { queue.enqueue_write_buffer(&mut a_mem_obj, CL_BLOCKING, 0, &A, &[])? };
    let _b_write_event =
        unsafe { queue.enqueue_write_buffer(&mut b_mem_obj, CL_BLOCKING, 0, &B, &[])? };

    // // Create a program from the kernel source
    // cl_program program = clCreateProgramWithSource(context, 1,
    //                                                (const char **)&source_str, (const size_t *)&source_size, &ret);

    let program = Program::create_and_build_from_source(&context, PROGRAM_SOURCE, "")
        .expect("Program::create_and_build_from_source failed");
    println!("{program:?}");

    // // Build the program
    // ret = clBuildProgram(program, 1, &device_id, NULL, NULL, NULL);

    // // Create the OpenCL kernel
    // cl_kernel kernel = clCreateKernel(program, "vector_add", &ret);

    let kernel = Kernel::create(&program, KERNEL_NAME).expect("Kernel::create failed");
    println!("{kernel:?}");

    // // Set the arguments of the kernel
    // ret = clSetKernelArg(kernel, 0, sizeof(cl_mem), (void *)&a_mem_obj);
    // ret = clSetKernelArg(kernel, 1, sizeof(cl_mem), (void *)&b_mem_obj);
    // ret = clSetKernelArg(kernel, 2, sizeof(cl_mem), (void *)&c_mem_obj);

    let kernel_event = unsafe {
        ExecuteKernel::new(&kernel)
            .set_arg(&a_mem_obj)
            .set_arg(&b_mem_obj)
            .set_arg(&c_mem_obj)
            .set_global_work_size(LIST_SIZE)
            .set_local_work_size(64)
            .enqueue_nd_range(&queue)?
    };
    println!("{kernel_event:?}");
    let mut events: Vec<cl_event> = Vec::default();
    events.push(kernel_event.get());

    // // Execute the OpenCL kernel on the list
    // size_t global_item_size = LIST_SIZE; // Process the entire lists
    // size_t local_item_size = 64; // Divide work items into groups of 64
    // ret = clEnqueueNDRangeKernel(command_queue, kernel, 1, NULL,
    //                              &global_item_size, &local_item_size, 0, NULL, NULL);

    // // Read the memory buffer C on the device to the local variable C
    // int *C = (int*)malloc(sizeof(int)*LIST_SIZE);
    // ret = clEnqueueReadBuffer(command_queue, c_mem_obj, CL_TRUE, 0,
    //                           LIST_SIZE * sizeof(int), C, 0, NULL, NULL);

    let mut results: [cl_int; LIST_SIZE] = [0; LIST_SIZE];
    let read_event =
        unsafe { queue.enqueue_read_buffer(&c_mem_obj, CL_BLOCKING, 0, &mut results, &events)? };

    // Wait for the read_event to complete.
    read_event.wait()?;

    // // Display the result to the screen
    // for(i = 0; i < LIST_SIZE; i++)
    // printf("%d + %d = %d\n", A[i], B[i], C[i]);
    //

    for i in 0..LIST_SIZE {
        println!("{} + {} = {}", A[i], B[i], results[i]);
    }

    // // Clean up
    // ret = clFlush(command_queue);
    // ret = clFinish(command_queue);
    // ret = clReleaseKernel(kernel);
    // ret = clReleaseProgram(program);
    // ret = clReleaseMemObject(a_mem_obj);
    // ret = clReleaseMemObject(b_mem_obj);
    // ret = clReleaseMemObject(c_mem_obj);
    // ret = clReleaseCommandQueue(command_queue);
    // ret = clReleaseContext(context);
    // free(A);
    // free(B);
    // free(C);

    Ok(())
}
