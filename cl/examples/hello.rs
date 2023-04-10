#![allow(non_camel_case_types, non_snake_case)]

use std::{mem, ptr};
use std::ffi::{c_char, c_void, CString};
// use cl3::command_queue::CL_QUEUE_PROFILING_ENABLE;
use cl3::context::CL_INVALID_VALUE;
use cl3::device::{get_device_ids, CL_DEVICE_TYPE_ALL};
use cl3::ext::{clBuildProgram, clCreateBuffer, clCreateCommandQueueWithProperties, clCreateContext, clCreateKernel, clCreateProgramWithSource, clEnqueueNDRangeKernel, clEnqueueWriteBuffer, clSetKernelArg};
use cl3::memory::{CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
use cl3::platform::get_platform_ids;
use cl3::types::{cl_context, cl_int, cl_command_queue, cl_float, cl_mem, CL_BLOCKING, cl_program, cl_uint, cl_kernel};
use cl::hello;

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

fn main() -> Result<(), cl_int> {
    println!("{}", hello());

    let platforms = get_platform_ids()?;
    println!("Number of platforms: {}", platforms.len());

    let platform_id = platforms.first().unwrap();
    println!("platform_id: {platform_id:#?}");

    let devices = get_device_ids(platform_id.clone(), CL_DEVICE_TYPE_ALL)?;
    println!("Number of devices: {}", devices.len());

    let device_id = devices.first().unwrap();
    println!("device_id: {device_id:#?}");

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

    let mut A: Vec<usize> = Vec::new();
    let mut B: Vec<usize> = Vec::new();

    for i in 0..LIST_SIZE {
        A.push(i);
        B.push(LIST_SIZE - i);
    }

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
    //

    let mut status: cl_int = CL_INVALID_VALUE;
    let context: cl_context;
    unsafe {
        context = clCreateContext(ptr::null(), 1, device_id, None, ptr::null_mut(), &mut status);
    }

    println!("clCreateContext: {status}");


    // // Create a command queue
    // cl_command_queue command_queue = clCreateCommandQueue(context, device_id, 0, &ret);

    let command_queue: cl_command_queue;
    unsafe {
        command_queue = clCreateCommandQueueWithProperties(context, *device_id, ptr::null(), &mut status);
    }

    println!("clCreateCommandQueueWithProperties: {status}");

    // // Create memory buffers on the device for each vector
    // cl_mem a_mem_obj = clCreateBuffer(context, CL_MEM_READ_ONLY,
    //                                   LIST_SIZE * sizeof(int), NULL, &ret);
    // cl_mem b_mem_obj = clCreateBuffer(context, CL_MEM_READ_ONLY,
    //                                   LIST_SIZE * sizeof(int), NULL, &ret);
    // cl_mem c_mem_obj = clCreateBuffer(context, CL_MEM_WRITE_ONLY,

    let a_mem_obj: cl_mem;
    let b_mem_obj: cl_mem;
    let c_mem_obj: cl_mem;
    unsafe {
        a_mem_obj = clCreateBuffer(context, CL_MEM_READ_ONLY, LIST_SIZE * mem::size_of::<cl_int>(), ptr::null_mut(), &mut status);
        println!("clCreateBuffer a: {status}");
        b_mem_obj = clCreateBuffer(context, CL_MEM_READ_ONLY, LIST_SIZE * mem::size_of::<cl_int>(), ptr::null_mut(), &mut status);
        println!("clCreateBuffer b: {status}");
        c_mem_obj = clCreateBuffer(context, CL_MEM_WRITE_ONLY, LIST_SIZE * mem::size_of::<cl_int>(), ptr::null_mut(), &mut status);
        println!("clCreateBuffer c: {status}");
    }


    // // Copy the lists A and B to their respective memory buffers
    // ret = clEnqueueWriteBuffer(command_queue, a_mem_obj, CL_TRUE, 0,
    //                            LIST_SIZE * sizeof(int), A, 0, NULL, NULL);
    // ret = clEnqueueWriteBuffer(command_queue, b_mem_obj, CL_TRUE, 0,
    //                            LIST_SIZE * sizeof(int), B, 0, NULL, NULL);

    unsafe {
        status = clEnqueueWriteBuffer(
            command_queue,
            a_mem_obj,
            CL_BLOCKING,
            0,
            LIST_SIZE * mem::size_of::<cl_int>(),
            A.as_ptr() as cl_mem,
            // A.as_ptr() as *const c_void,
            0,
            ptr::null(),
            ptr::null_mut()
        );
        println!("clEnqueueWriteBuffer a: {status}");

        status = clEnqueueWriteBuffer(
            command_queue,
            b_mem_obj,
            CL_BLOCKING,
            0,
            LIST_SIZE * mem::size_of::<cl_int>(),
            B.as_ptr() as cl_mem,
            0,
            ptr::null(),
            ptr::null_mut()
        );
        println!("clEnqueueWriteBuffer b: {status}");
    }


    // // Create a program from the kernel source
    // cl_program program = clCreateProgramWithSource(context, 1,
    //                                                (const char **)&source_str, (const size_t *)&source_size, &ret);

    let program: cl_program;
    unsafe {
        let sources = [PROGRAM_SOURCE];
        let lengths: Vec<usize> = sources.iter().map(|src| src.len()).collect();
        program = clCreateProgramWithSource(
            context,
            1,
            sources.as_ptr() as *const *const c_char,
            lengths.as_ptr(),
            &mut status,
        );
    }

    println!("clCreateProgramWithSource: {status}");

    // // Build the program
    // ret = clBuildProgram(program, 1, &device_id, NULL, NULL, NULL);

    status = unsafe {
        clBuildProgram(
            program,
            devices.len() as cl_uint,
            devices.as_ptr(),
            ptr::null(),
            None,
            ptr::null_mut(),
        )
    };

    println!("clBuildProgram: {status}");

    // // Create the OpenCL kernel
    // cl_kernel kernel = clCreateKernel(program, "vector_add", &ret);
    let c_name = CString::new(KERNEL_NAME).expect("Kernel::create, invalid name");
    let kernel: cl_kernel = unsafe {
        clCreateKernel(program, c_name.as_ptr(), &mut status)
    };

    println!("clCreateKernel: {status}");

    // // Set the arguments of the kernel
    // ret = clSetKernelArg(kernel, 0, sizeof(cl_mem), (void *)&a_mem_obj);
    // ret = clSetKernelArg(kernel, 1, sizeof(cl_mem), (void *)&b_mem_obj);
    // ret = clSetKernelArg(kernel, 2, sizeof(cl_mem), (void *)&c_mem_obj);

    status = unsafe {
        clSetKernelArg(
            kernel,
            0 as cl_uint,
            mem::size_of::<cl_mem>(),
            ptr::null()
        )
    };
    println!("clSetKernelArg a: {status}");

    status = unsafe {
        clSetKernelArg(
            kernel,
            1 as cl_uint,
            mem::size_of::<cl_mem>(),
            ptr::null()
        )
    };
    println!("clSetKernelArg b: {status}");

    status = unsafe {
        clSetKernelArg(
            kernel,
            2 as cl_uint,
            mem::size_of::<cl_mem>(),
            ptr::null()
        )
    };
    println!("clSetKernelArg c: {status}");

    // // Execute the OpenCL kernel on the list
    // size_t global_item_size = LIST_SIZE; // Process the entire lists
    // size_t local_item_size = 64; // Divide work items into groups of 64
    // ret = clEnqueueNDRangeKernel(command_queue, kernel, 1, NULL,
    //                              &global_item_size, &local_item_size, 0, NULL, NULL);

    let global_item_size = LIST_SIZE;
    let local_item_size: usize = 64;
    status = unsafe {
        clEnqueueNDRangeKernel(
            command_queue,
            kernel,
            1 as cl_uint,
            ptr::null(),
            global_item_size as *const usize,
            local_item_size as *const usize,
            0,
            ptr::null(),
            ptr::null_mut()
        )
    };
    println!("clEnqueueNDRangeKernel: {status}");


    // // Read the memory buffer C on the device to the local variable C
    // int *C = (int*)malloc(sizeof(int)*LIST_SIZE);
    // ret = clEnqueueReadBuffer(command_queue, c_mem_obj, CL_TRUE, 0,
    //                           LIST_SIZE * sizeof(int), C, 0, NULL, NULL);
    //
    // // Display the result to the screen
    // for(i = 0; i < LIST_SIZE; i++)
    // printf("%d + %d = %d\n", A[i], B[i], C[i]);
    //
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