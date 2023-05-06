use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, Pipe, CL_MEM_READ_ONLY, CL_MEM_READ_WRITE, CL_MEM_WRITE_ONLY};
use opencl3::program::{Program, CL_STD_2_0};
use opencl3::types::{cl_event, cl_int, cl_uint, CL_BLOCKING};
use opencl3::Result;
use std::{mem, ptr};

const PRODUCER_KERNEL_NAME: &str = "producer";
const CONSUMER_KERNEL_NAME: &str = "consumer";

// const KB_8: usize = 8192; // default ReadBuffer

pub const KB_1: usize = 1024; // 1024
pub const KB_N: usize = KB_1 * 1;
pub const LIST_SIZE: usize = KB_N;
pub const PIPE_MAX_PACKETS: usize = 1024;
pub const PIPE_BLOCKS: usize = 1;
pub const PIPE_MAX_CAP: usize = KB_N * PIPE_MAX_PACKETS;

// CONFIG A =
// buffer_capacity: 1 kb - pipe_capacity: 8 kb
// memory_store: 5.12 MB - memory_required: 2.560 GB
// pipe_blocks = 640

// pub const KB_1: usize = 1024;
// pub const KB_N: usize = KB_1 * 1;
// pub const LIST_SIZE: usize = KB_N;
// pub const PIPE_MAX_PACKETS: usize = 8;
// pub const PIPE_BLOCKS: u32 = 640;
// pub const PIPE_MAX_CAP: usize = KB_N * PIPE_MAX_PACKETS;

// CONFIG B =
// buffer_capacity: 2 kb - pipe_capacity: 8 kb
// memory_store: 2.5 MB - memory_required: 2.580 GB
// pipe_blocks = 320

// const KB_1: usize = 1024;
// const KB_N: usize = KB_1 * 2;
// pub const LIST_SIZE: usize = KB_N;
// pub const PIPE_MAX_PACKETS: u32 = 4;
// pub const PIPE_BLOCKS: u32 = 320;

#[derive(Debug, Clone)]
pub struct BlockConfig {
    pub buffer_size: usize, // error with constants
    pub pipes: usize,
    pub pipe_max_packets: usize,
}

#[derive(Debug)]
pub struct OpenClBlock {
    context: Context,
    producer_kernel: Kernel,
    consumer_kernel: Kernel,
    queue: CommandQueue,
    config: BlockConfig,
}

impl OpenClBlock {
    pub fn new(config: BlockConfig) -> Result<OpenClBlock> {
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

        let producer_program_source = generate_producer_source(config.pipes);
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

        let consumer_program_source = generate_consumer_source(config.pipes);
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
            config,
        })
    }

    pub fn generate_pipes(&self, len: usize) -> Result<Vec<PipeBlock>> {
        let mut vec = vec![];

        for i in 0..len {
            let mut pipe_vec = vec![];
            for _ in 0..self.config.pipes {
                pipe_vec.push(unsafe {
                    Pipe::create(
                        &self.context,
                        CL_MEM_READ_WRITE,
                        mem::size_of::<cl_int>() as cl_uint,
                        self.config.pipe_max_packets as cl_uint,
                    )
                    .expect("Pipe::create failed")
                })
            }
            // println!("{pipe_vec:?}");
            println!("pipe_vec block {i}");

            let pipe_block = PipeBlock {
                context: &self.context,
                producer_kernel: &self.producer_kernel,
                consumer_kernel: &self.consumer_kernel,
                queue: &self.queue,
                pipe_vec,
                config: self.config.clone(),
                saved_index: 0,
            };

            vec.push(pipe_block)
        }

        Ok(vec)
    }
}

#[derive(Debug)]
pub struct PipeBlock<'a> {
    context: &'a Context,
    producer_kernel: &'a Kernel,
    consumer_kernel: &'a Kernel,
    queue: &'a CommandQueue,
    pipe_vec: Vec<Pipe>,
    config: BlockConfig,
    saved_index: u64,
}

impl<'a> PipeBlock<'a> {
    pub fn enqueue(&self, buf: &[u8]) -> Result<()> {
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
        let mut input_mem_obj = unsafe {
            Buffer::<cl_int>::create(
                self.context,
                CL_MEM_READ_ONLY,
                self.config.buffer_size,
                ptr::null_mut(),
            )?
        };

        let _write_event = unsafe {
            &self
                .queue
                .enqueue_write_buffer(&mut input_mem_obj, CL_BLOCKING, 0, &input, &[])?
        };

        let event = unsafe {
            let mut ex = ExecuteKernel::new(self.producer_kernel);

            ex.set_arg(&input_mem_obj);

            for p in self.pipe_vec.iter() {
                ex.set_arg(p);
            }

            ex.set_global_work_size(self.config.buffer_size)
                .set_local_work_size(64)
                .enqueue_nd_range(self.queue)?
        };

        event.wait().expect("event.wait");

        Ok(())
    }

    pub fn enqueue_v2(&mut self, buf: &[u8]) -> Result<()> {
        let chunks = buf.chunks(LIST_SIZE);
        println!("chunks: {}", chunks.len());
        if chunks.len() > PIPE_MAX_PACKETS {
            println!("buffer too large");
        }

        let mut input_mem_obj = unsafe {
            Buffer::<cl_int>::create(
                self.context,
                CL_MEM_READ_ONLY,
                self.config.buffer_size,
                ptr::null_mut(),
            )?
        };

        self.saved_index += chunks.len() as u64;

        for chunk in chunks {
            println!("save chunk {}", chunk.len());
            let mut input: [cl_int; LIST_SIZE] = [-1; LIST_SIZE];

            if LIST_SIZE > chunk.len() {
                for (i, v) in chunk.iter().enumerate() {
                    input[i] = *v as cl_int;
                }
            } else {
                for i in 0..LIST_SIZE {
                    input[i] = chunk[i] as cl_int;
                }
            }

            let _write_event = unsafe {
                &self
                    .queue
                    .enqueue_write_buffer(&mut input_mem_obj, CL_BLOCKING, 0, &input, &[])?
            };

            let event = unsafe {
                let mut ex = ExecuteKernel::new(self.producer_kernel);

                ex.set_arg(&input_mem_obj);

                for p in self.pipe_vec.iter() {
                    ex.set_arg(p);
                }

                ex.set_global_work_size(self.config.buffer_size)
                    .set_local_work_size(64)
                    .enqueue_nd_range(self.queue)?
            };

            event.wait().expect("event.wait");
        }

        Ok(())
    }

    pub fn dequeue(&self) -> Result<Vec<u8>> {
        let output_mem_obj = unsafe {
            Buffer::<cl_int>::create(
                self.context,
                CL_MEM_WRITE_ONLY,
                self.config.buffer_size,
                ptr::null_mut(),
            )?
        };

        let kernel_event = unsafe {
            let mut ex = ExecuteKernel::new(self.consumer_kernel);

            ex.set_arg(&output_mem_obj);

            for p in self.pipe_vec.iter() {
                ex.set_arg(p);
            }

            ex.set_global_work_size(self.config.buffer_size)
                .set_local_work_size(64)
                .enqueue_nd_range(self.queue)?
        };

        let mut events: Vec<cl_event> = Vec::default();
        events.push(kernel_event.get());

        let mut output: [cl_int; LIST_SIZE] = [0; LIST_SIZE];

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

        let display = LIST_SIZE;
        let mut output_vec = vec![];

        for i in 0..display {
            // output_vec.push(output[i]);
            // println!("{}", output[i]);
            if output[i] > -1 {
                // println!("i{} + v{}", i, output[i]);
                output_vec.push(output[i] as u8);
            }
        }

        // println!("output {output:?}");
        // println!(
        //     "consume pipe: {}",
        //     String::from_utf8(output_vec.clone()).expect("from_utf8")
        // );

        // Ok(output)
        Ok(output_vec)
    }

    pub fn dequeue_v2(&mut self) -> Result<Vec<u8>> {
        println!("saved_index {}", self.saved_index);
        let output_mem_obj = unsafe {
            Buffer::<cl_int>::create(
                self.context,
                CL_MEM_WRITE_ONLY,
                self.config.buffer_size,
                ptr::null_mut(),
            )?
        };
        let mut vec: Vec<u8> = Vec::with_capacity(PIPE_MAX_CAP);
        let mut output: [cl_int; LIST_SIZE] = [-1; LIST_SIZE];

        for i in 0..self.saved_index {
            println!("pipe index {i}");
            let kernel_event = unsafe {
                let mut ex = ExecuteKernel::new(self.consumer_kernel);

                ex.set_arg(&output_mem_obj);

                for p in self.pipe_vec.iter() {
                    ex.set_arg(p);
                }

                ex.set_global_work_size(self.config.buffer_size)
                    .set_local_work_size(64)
                    .enqueue_nd_range(self.queue)?
            };

            let mut events: Vec<cl_event> = Vec::default();
            events.push(kernel_event.get());

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

            let mut output_vec = Vec::with_capacity(LIST_SIZE);

            for i in 0..LIST_SIZE {
                if output[i] > -1 {
                    output_vec.push(output[i] as u8);
                }
            }

            vec.append(&mut output_vec);
        }

        println!(
            "consume pipe: {}",
            String::from_utf8(vec.clone()).expect("from_utf8")
        );

        self.saved_index = 0;

        // Ok(output)
        Ok(vec)
    }
}

impl<'a> Iterator for PipeBlock<'a> {
    // We can refer to this type using Self::Item
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dequeue() {
            Ok(vec) => {
                if vec.len() == 0 {
                    return None;
                }
                Some(vec)
            }
            Err(e) => {
                panic!("opencl_error {e}")
            }
        }
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn enqueue_buffer_1_kb() {
//         const SIZE: usize = 5;
//         let a: [u8; SIZE] = [0; SIZE];
//
//         let ocl_block = OpenClBlock::new(BlockConfig {
//             buffer_size: KB_N,
//             pipes: KB_N,
//             pipe_max_packets: PIPE_MAX_PACKETS,
//         })
//             .expect("OpenClBlock::new()");
//         let mut pipe_blocks = ocl_block
//             .generate_pipes(PIPE_BLOCKS)
//             .expect("ocl_block.generate_pipes()");
//     }
// }
