use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, Pipe, CL_MEM_READ_ONLY, CL_MEM_READ_WRITE, CL_MEM_WRITE_ONLY};
use opencl3::program::{Program, CL_STD_2_0};
use opencl3::types::{cl_event, cl_int, cl_uint, CL_BLOCKING};
use opencl3::Result;
use std::time::Duration;
use std::{mem, ptr, thread};

const KB_1: usize = 1024;
const KB_N: usize = KB_1 * 1;
// const KB_8: usize = 8192;

const LIST_SIZE: usize = KB_N;
const TOTAL_PIPES: usize = KB_N;
const PIPE_LEN: usize = 10;
const SLEEP: bool = true;
const SLEEP_DURATION: u64 = 4000; // millis

fn gen_producer_source(pipes: usize) -> String {
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

const PRODUCER_KERNEL_NAME: &str = "producer";

fn gen_consumer_source(pipes: usize) -> String {
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

const CONSUMER_KERNEL_NAME: &str = "consumer";

fn produce(
    context: &Context,
    kernel: &Kernel,
    queue: &CommandQueue,
    pipe_vec: &Vec<Pipe>,
    values: Vec<u8>,
) -> Result<()> {
    let mut input: [cl_int; LIST_SIZE] = [-1; LIST_SIZE];
    println!("values vec len: {}", values.len());

    if LIST_SIZE > values.len() {
        for (i, v) in values.iter().enumerate() {
            input[i] = *v as cl_int;
        }
    } else {
        for i in 0..LIST_SIZE {
            input[i] = values[i] as cl_int;
        }
    }

    println!("input arr len: {}", input.len());
    // println!("input arr: {input:?}");

    let mut input_mem_obj =
        unsafe { Buffer::<cl_int>::create(context, CL_MEM_READ_ONLY, LIST_SIZE, ptr::null_mut())? };

    let _write_event =
        unsafe { queue.enqueue_write_buffer(&mut input_mem_obj, CL_BLOCKING, 0, &input, &[])? };

    let _ = unsafe {
        let mut ex = ExecuteKernel::new(kernel);

        ex.set_arg(&input_mem_obj);

        for p in pipe_vec.iter() {
            ex.set_arg(p);
        }

        ex.set_global_work_size(LIST_SIZE)
            .set_local_work_size(64)
            .enqueue_nd_range(queue)?
    };

    Ok(())
}

fn consume(
    context: &Context,
    kernel: &Kernel,
    queue: &CommandQueue,
    pipe_vec: &Vec<Pipe>,
) -> Result<[cl_int; LIST_SIZE]> {
    let output_mem_obj = unsafe {
        Buffer::<cl_int>::create(context, CL_MEM_WRITE_ONLY, LIST_SIZE, ptr::null_mut())?
    };

    let kernel_event = unsafe {
        let mut ex = ExecuteKernel::new(kernel);

        ex.set_arg(&output_mem_obj);

        for p in pipe_vec.iter() {
            ex.set_arg(p);
        }

        ex.set_global_work_size(LIST_SIZE)
            .set_local_work_size(64)
            .enqueue_nd_range(queue)?
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
    let mut output_vec = vec![];

    println!("kernel pipe output");
    for i in 0..display {
        // println!("{}", output[i]);
        if output[i] > -1 {
            // println!("i{} + v{}", i, output[i]);
            output_vec.push(output[i] as u8);
        }
    }

    println!(
        "consume pipe: {}",
        String::from_utf8(output_vec).expect("from_utf8")
    );

    Ok(output)
}

struct OpenClObject {
    context: Context,
    producer_kernel: Kernel,
    consumer_kernel: Kernel,
    queue: CommandQueue,
    pipe_vec: Vec<Pipe>,
}

fn ocl_pipe() -> Result<OpenClObject> {
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
                // (LIST_SIZE * mem::size_of::<cl_uchar>()) as cl_uint,
                // (LIST_SIZE * 10) as cl_uint,
                // LIST_SIZE as cl_uint,
                8,
            )
            .expect("Pipe::create failed")
        })
    }
    // println!("{pipe_vec:?}");
    println!("pipe_vec");

    let producer_program_source = gen_producer_source(TOTAL_PIPES);
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

    let consumer_program_source = gen_consumer_source(TOTAL_PIPES);
    let consumer_program = Program::create_and_build_from_source(
        &context,
        consumer_program_source.as_str(),
        "-cl-std=CL2.0",
    )
    .expect("Program::create_and_build_from_source failed");
    println!("{consumer_program:?}");

    let consumer_kernel =
        Kernel::create(&consumer_program, CONSUMER_KERNEL_NAME).expect("Kernel::create failed");
    println!("{consumer_kernel:?}");

    Ok(OpenClObject {
        context,
        producer_kernel,
        consumer_kernel,
        queue,
        pipe_vec,
    })
}

fn add_text(
    context: &Context,
    kernel: &Kernel,
    queue: &CommandQueue,
    pipe_vec: &Vec<Pipe>,
) -> Result<usize> {
    let values = [
        String::from("hello").as_bytes().to_owned(),
        String::from("world").as_bytes().to_owned(),
        String::from("large_text_____________________________________________________________________________here........").as_bytes().to_owned(),
        String::from("text_batch_____________________________________________________________________________here....._2").as_bytes().to_owned(),
        String::from("Lorem Ipsum is simply dummy text of the printing and typesetting industry").as_bytes().to_owned(),
        String::from("It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout. The point of using Lorem Ipsum is that it has a more-or-less normal distribution of letters, as opposed to using 'Content here, content here', making it look like readable English. Many desktop publishing packages and web page editors now use Lorem Ipsum as their default model text, and a search for 'lorem ipsum' will uncover many web sites still in their infancy. Various versions have evolved over the years, sometimes by accident, sometimes on purpose (injected humour and the like)").as_bytes().to_owned(),
    ];
    let len = values.len();

    // produce
    for value in values {
        println!("produce -----------------------");
        produce(context, kernel, queue, pipe_vec, value)?;
        println!("produce -----------------------");
    }

    Ok(len)
}

fn consume_pipe(
    context: &Context,
    kernel: &Kernel,
    queue: &CommandQueue,
    pipe_vec: &Vec<Pipe>,
    len: usize,
) -> Result<()> {
    for _ in 0..len {
        consume(context, kernel, queue, pipe_vec)?;
    }
    Ok(())
}

fn main() {
    let threads: Vec<_> = (0..1)
        .map(|i| {
            thread::spawn(move || {
                println!("Thread #{i} started!");
                let OpenClObject {
                    context,
                    producer_kernel,
                    consumer_kernel,
                    queue,
                    pipe_vec,
                } = ocl_pipe().unwrap();
                let total = add_text(&context, &producer_kernel, &queue, &pipe_vec).unwrap();
                thread::sleep(Duration::from_millis(500));
                consume_pipe(&context, &consumer_kernel, &queue, &pipe_vec, total).unwrap();
                println!("Thread #{i} finished!");
            })
        })
        .collect();

    for handle in threads {
        handle.join().unwrap();
    }

    thread::sleep(Duration::from_millis(1000));
    println!("Thread #main finished!");
}
