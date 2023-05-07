use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::error_codes::{CL_MEM_OBJECT_ALLOCATION_FAILURE, ClError};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
use opencl3::program::{Program, CL_STD_2_0};
use opencl3::types::{cl_event, cl_int, CL_BLOCKING};
use opencl3::Result;
use std::fs::DirEntry;
use std::path::Path;
use std::{fs, io, ptr};

pub const KB_1: usize = 1024; // 1024
pub const MB_1: usize = KB_1 * KB_1;

pub const LIST_SIZE: usize = MB_1;
// pub const BLOCKS: usize = 1;
pub const TOTAL_GLOBAL_ARRAY: usize = 512;
pub const CAPACITY_GLOBAL_ARRAY: usize = MB_1 * 1;

pub const KERNEL_NAME: &str = "vector_add";
pub const EXTRACT_KERNEL_NAME: &str = "vector_extract";

#[derive(Debug)]
pub struct OpenClBlock {
    context: Context,
    vector_add_kernel: Kernel,
    vector_extract_kernel: Kernel,
    queue: CommandQueue,
}

impl OpenClBlock {
    pub fn new() -> Result<OpenClBlock> {
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

        let program_source = gen_vector_program_source(TOTAL_GLOBAL_ARRAY, CAPACITY_GLOBAL_ARRAY);
        // println!("{program_source}");
        let program =
            Program::create_and_build_from_source(&context, program_source.as_str(), CL_STD_2_0)
                .expect("Program::create_and_build_from_source failed");
        println!("{program:?}");

        let vector_add_kernel =
            Kernel::create(&program, KERNEL_NAME).expect("Kernel::create failed");
        println!("{vector_add_kernel:?}");

        let vector_extract_kernel =
            Kernel::create(&program, EXTRACT_KERNEL_NAME).expect("Kernel::create failed");
        println!("{vector_extract_kernel:?}");

        Ok(OpenClBlock {
            context,
            vector_add_kernel,
            vector_extract_kernel,
            queue,
        })
    }

    pub fn enqueue_buffer(&mut self, buf: &[u8], global_array_index: cl_int) -> Result<()> {
        if buf.len() > LIST_SIZE {
            println!("buffer too large");
            return Err(ClError(CL_MEM_OBJECT_ALLOCATION_FAILURE));
        }

        let mut input_mem_obj = unsafe {
            Buffer::<cl_int>::create(&self.context, CL_MEM_READ_ONLY, LIST_SIZE, ptr::null_mut())?
        };

        // select global array
        let d: [cl_int; LIST_SIZE] = [global_array_index; LIST_SIZE];
        let mut d_mem_obj = unsafe {
            Buffer::<cl_int>::create(&self.context, CL_MEM_READ_ONLY, LIST_SIZE, ptr::null_mut())?
        };
        let _d_write_event = unsafe {
            &self
                .queue
                .enqueue_write_buffer(&mut d_mem_obj, CL_BLOCKING, 0, &d, &[])?
        };

        let mut input: [cl_int; LIST_SIZE] = [-1; LIST_SIZE];

        if LIST_SIZE > buf.len() {
            for (i, v) in buf.iter().enumerate() {
                input[i] = *v as cl_int;
            }
        } else {
            for i in 0..LIST_SIZE {
                input[i] = buf[i] as cl_int;
            }
        }

        let _write_event = unsafe {
            &self
                .queue
                .enqueue_write_buffer(&mut input_mem_obj, CL_BLOCKING, 0, &input, &[])?
        };

        let event = unsafe {
            let mut ex = ExecuteKernel::new(&self.vector_add_kernel);

            ex.set_arg(&input_mem_obj).set_arg(&d_mem_obj);

            ex.set_global_work_size(LIST_SIZE)
                .set_local_work_size(64)
                .enqueue_nd_range(&self.queue)?
        };

        event.wait().expect("event.wait");

        Ok(())
    }

    pub fn dequeue_buffer(&self, global_array_index: cl_int) -> Result<Vec<u8>> {
        let output_mem_obj = unsafe {
            Buffer::<cl_int>::create(&self.context, CL_MEM_WRITE_ONLY, LIST_SIZE, ptr::null_mut())?
        };

        // select global array
        let d: [cl_int; LIST_SIZE] = [global_array_index; LIST_SIZE];
        let mut d_mem_obj = unsafe {
            Buffer::<cl_int>::create(&self.context, CL_MEM_READ_ONLY, LIST_SIZE, ptr::null_mut())?
        };
        let _d_write_event = unsafe {
            &self
                .queue
                .enqueue_write_buffer(&mut d_mem_obj, CL_BLOCKING, 0, &d, &[])?
        };

        let kernel_event = unsafe {
            let mut ex = ExecuteKernel::new(&self.vector_extract_kernel);

            ex.set_arg(&output_mem_obj).set_arg(&d_mem_obj);

            ex.set_global_work_size(LIST_SIZE)
                .set_local_work_size(64)
                .enqueue_nd_range(&self.queue)?
        };

        let mut events: Vec<cl_event> = Vec::default();
        events.push(kernel_event.get());

        let mut output: [cl_int; LIST_SIZE] = [-1; LIST_SIZE];

        let _read_event = unsafe {
            &self.queue.enqueue_read_buffer(
                &output_mem_obj,
                CL_BLOCKING,
                0,
                &mut output,
                &events,
            )?
        };

        // Wait for the read_event to complete.
        // read_event.wait()?;

        let mut output_vec = Vec::with_capacity(LIST_SIZE as usize);

        for i in 0..LIST_SIZE {
            // output_vec.push(output[i]);
            // println!("{}", output[i]);
            if output[i] > -1 {
                // println!("i{} + v{}", i, output[i]);
                output_vec.push(output[i] as u8);
            }
        }

        // println!("output {output:?}");
        println!(
            "consume arr: {}",
            String::from_utf8(output_vec.clone()).expect("from_utf8")
        );

        // Ok(output)
        Ok(output_vec)
    }
}

pub fn gen_vector_program_source(arrays: usize, capacity: usize) -> String {
    let mut global_arrays = String::from(
        "
        ",
    );

    let mut vector_add_fn = String::from(
        "
    kernel void vector_add(global int* A, global int* D) {

        // Get the index of the current element
        int i = get_global_id(0);

        switch (D[i]) {
        ",
    );

    let mut vector_extract_fn = String::from(
        "
    kernel void vector_extract(global int* C, global int* D) {

        // Get the index of the current element
        int i = get_global_id(0);

        switch (D[i]) {
        ",
    );

    for i in 0..arrays {
        // global_array
        let global_arr = format!(
            "
    __global int myNumbers{i}[{capacity}];"
        );
        global_arrays.push_str(&global_arr);

        // vector_add
        // let v_add = format!("
        // myNumbers{i}[i] = A[i] + {i};"
        // );
        let v_add = format!(
            "
            case {i}:
              myNumbers{i}[i] = A[i];
              break;"
        );
        vector_add_fn.push_str(&v_add);

        // vector_extract
        let v_ext = format!(
            "
            case {i}:
              C[i] = myNumbers{i}[i];
              break;"
        );
        vector_extract_fn.push_str(&v_ext);
    }

    let end_switch = r#"
        }"#;

    let end_fn = r#"
    }"#;

    let body = format!(
        "
    {vector_add_fn}
    {end_switch}
    {end_fn}
    {vector_extract_fn}
    {end_switch}
    {end_fn}
    "
    );

    format!("{global_arrays}{body}")
}

pub fn visit_dirs(dir: &Path, vec: &mut Vec<DirEntry>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, vec)?;
            } else {
                vec.push(entry);
            }
        }
    }
    Ok(())
}
