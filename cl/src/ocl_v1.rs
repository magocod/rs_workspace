use cl3::ext::cl_char;
use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, Pipe, CL_MEM_READ_ONLY, CL_MEM_READ_WRITE, CL_MEM_WRITE_ONLY};
use opencl3::program::{Program, CL_STD_2_0};
use opencl3::types::{cl_event, cl_int, cl_uchar, cl_uint, CL_BLOCKING};
use opencl3::Result;
use std::{mem, ptr};

const PRODUCER_KERNEL_NAME: &str = "producer";
const CONSUMER_KERNEL_NAME: &str = "consumer";

const KB_1: usize = 1024;
const KB_N: usize = KB_1 * 1;
// const KB_8: usize = 8192; // default ReadBuffer

const LIST_SIZE: usize = KB_N;
const TOTAL_PIPES: usize = KB_N;
const PIPE_MAX_PACKETS: u32 = 8; // 8 kb

pub struct OpenClBlock {
    context: Context,
    producer_kernel: Kernel,
    consumer_kernel: Kernel,
    queue: CommandQueue,
    pipe_vec: Vec<Pipe>,
}

pub type ClArray = [cl_int; LIST_SIZE];

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

        let mut pipe_vec = Vec::with_capacity(TOTAL_PIPES);
        for _ in 0..TOTAL_PIPES {
            pipe_vec.push(unsafe {
                Pipe::create(
                    &context,
                    CL_MEM_READ_WRITE,
                    // (LIST_SIZE * mem::size_of::<cl_int>()) as cl_uint,
                    mem::size_of::<cl_int>() as cl_uint,
                    PIPE_MAX_PACKETS,
                )
                .expect("Pipe::create failed")
            })
        }
        // println!("{pipe_vec:?}");
        println!("pipe_vec");

        let producer_program_source = generate_producer_source(TOTAL_PIPES);
        let producer_program = Program::create_and_build_from_source(
            &context,
            producer_program_source.as_str(),
            CL_STD_2_0,
        )
        .expect("Program::create_and_build_from_source failed");
        println!("{producer_program:?}");

        let producer_kernel =
            Kernel::create(&producer_program, PRODUCER_KERNEL_NAME).expect("Kernel::create failed");
        println!("{producer_kernel:?}");

        let consumer_program_source = generate_consumer_source(TOTAL_PIPES);
        let consumer_program = Program::create_and_build_from_source(
            &context,
            consumer_program_source.as_str(),
            CL_STD_2_0,
        )
        .expect("Program::create_and_build_from_source failed");
        println!("{consumer_program:?}");

        let consumer_kernel =
            Kernel::create(&consumer_program, CONSUMER_KERNEL_NAME).expect("Kernel::create failed");
        println!("{consumer_kernel:?}");

        Ok(OpenClBlock {
            context,
            producer_kernel,
            consumer_kernel,
            queue,
            pipe_vec,
        })
    }

    // ERROR don't work
    pub fn enqueue(&self, buf: &[cl_uchar]) -> Result<()> {
        let mut input_mem_obj = unsafe {
            Buffer::<cl_uchar>::create(&self.context, CL_MEM_READ_ONLY, LIST_SIZE, ptr::null_mut())?
        };

        let _write_event = unsafe {
            &self
                .queue
                .enqueue_write_buffer(&mut input_mem_obj, CL_BLOCKING, 0, buf, &[])?
        };

        let event = unsafe {
            let mut ex = ExecuteKernel::new(&self.producer_kernel);

            ex.set_arg(&input_mem_obj);

            for p in self.pipe_vec.iter() {
                ex.set_arg(p);
            }

            ex.set_global_work_size(LIST_SIZE)
                .set_local_work_size(64)
                .enqueue_nd_range(&self.queue)?
        };

        event.wait().expect("event.wait");

        Ok(())
    }

    // ERROR don't work
    pub fn dequeue(&self) -> Result<Vec<u8>> {
        // ERROR HERE cl_char
        let output_mem_obj = unsafe {
            Buffer::<cl_char>::create(&self.context, CL_MEM_WRITE_ONLY, LIST_SIZE, ptr::null_mut())?
        };

        let kernel_event = unsafe {
            let mut ex = ExecuteKernel::new(&self.consumer_kernel);

            ex.set_arg(&output_mem_obj);

            for p in self.pipe_vec.iter() {
                ex.set_arg(p);
            }

            ex.set_global_work_size(LIST_SIZE)
                .set_local_work_size(64)
                .enqueue_nd_range(&self.queue)?
        };

        let mut events: Vec<cl_event> = Vec::default();
        events.push(kernel_event.get());

        let mut output: [cl_char; LIST_SIZE] = [0; LIST_SIZE];

        let read_event = unsafe {
            &self.queue.enqueue_read_buffer(
                &output_mem_obj,
                CL_BLOCKING,
                0,
                &mut output,
                &events,
            )?
        };

        // Wait for the read_event to complete.
        read_event.wait()?;

        let display = LIST_SIZE;
        let mut output_vec = vec![];

        println!("kernel pipe output");
        for i in 0..display {
            // output_vec.push(output[i]);
            // println!("{}", output[i]);
            if output[i] > -1 {
                // println!("i{} + v{}", i, output[i]);
                output_vec.push(output[i] as u8);
            }
        }

        // println!("output {output:?}");
        println!(
            "consume pipe: {}",
            String::from_utf8(output_vec.clone()).expect("from_utf8")
        );

        // Ok(output)
        Ok(output_vec)
    }
}

pub fn generate_producer_source(pipes: usize) -> String {
    let header = r#"
    __kernel void producer(
        __global int* input,"#;

    let mut body_pipe_args = String::from(
        "
        ",
    );

    let mut body_pipes = String::from(
        "
        switch (i) {
        ",
    );

    for i in 0..pipes {
        let v = format!(
            "
            __write_only pipe int p{i},"
        );
        body_pipe_args.push_str(&v);
        let v = format!(
            "
            case {i}:
              write_pipe(p{i}, &input[i]);
              break;
            "
        );
        body_pipes.push_str(&v);
    }

    body_pipe_args.pop(); // remove last ( , ) example: "int p0, int p1"
    body_pipes.push_str(
        "
        }
    ",
    );

    let body_get_global_id = r#"
        ) {

        // Get the index of the current element
        int i = get_global_id(0);
        // printf("global_id %d %d",  i, input[i]);
        "#;

    let body = format!("{body_pipe_args}{body_get_global_id}{body_pipes}");

    let end = r#"
    }"#;

    format!("{header}{body}{end}")
}

pub fn generate_consumer_source(pipes: usize) -> String {
    let header = r#"
    __kernel void consumer(
        __global int* output,"#;

    let mut body_pipe_args = String::from(
        "
        ",
    );

    let mut body_pipes = String::from(
        "
        switch (i) {
        ",
    );

    for i in 0..pipes {
        let v = format!(
            "
            __read_only pipe int p{i},"
        );
        body_pipe_args.push_str(&v);
        let v = format!(
            "
            case {i}:
              read_pipe(p{i}, &output[i]);
              break;
            "
        );
        body_pipes.push_str(&v);
    }

    body_pipe_args.pop(); // remove last ( , ) example: "int p0, int p1"
    body_pipes.push_str(
        "
        }
    ",
    );

    let body_get_global_id = r#"
        ) {

        // Get the index of the current element
        int i = get_global_id(0);
        // printf("global_id %d %d",  i, input[i]);

        output[i] = -1;
        "#;

    let body = format!("{body_pipe_args}{body_get_global_id}{body_pipes}");

    let end = r#"
    }"#;

    format!("{header}{body}{end}")
}
