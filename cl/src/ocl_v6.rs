// use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
// use opencl3::context::Context;
// use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
// use opencl3::kernel::{ExecuteKernel, Kernel};
// use opencl3::memory::{Buffer, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
// use opencl3::program::{Program, CL_STD_2_0};
// use opencl3::types::{cl_event, cl_int, CL_BLOCKING};
// // use opencl3::Result;
// use std::collections::HashMap;
// use std::fs::DirEntry;
// use std::path::Path;
// use std::{fs, io, ptr};
// // use cl3::error_codes::ClError;
// use crate::error::{
//     OpenClResult,
//     OpenclError,
//     // GLOBAL_ARRAY_ID_ASSIGNED,
//     // INVALID_BUFFER_LEN,
//     INVALID_GLOBAL_ARRAY_ID,
//     NO_GLOBAL_VECTORS_TO_ASSIGN,
// };
//
pub const KB_1: usize = 1024; // 1024
pub const MB_1: usize = KB_1 * KB_1;
// pub const SIZE: usize = KB_1 * 1;
//
// pub const LIST_SIZE: usize = SIZE;
// // pub const BLOCKS: usize = 1;
// pub const TOTAL_GLOBAL_ARRAY: usize = 2500;
// pub const CAPACITY_GLOBAL_ARRAY: usize = SIZE;

pub const VECTOR_ADD_KERNEL_NAME: &str = "vector_add";
pub const VECTOR_EXTRACT_KERNEL_NAME: &str = "vector_extract";

// pub type GlobalArrayMap = HashMap<u32, u64>;
//
// #[derive(Debug)]
// pub struct OpenClBlock {
//     context: Context,
//     // safe Send & Sync
//     // vector_add_kernel: Kernel,
//     // vector_extract_kernel: Kernel,
//     queue: CommandQueue,
//     program: Program,
//     global_arrays: GlobalArrayMap,
//     // config
//     vector_size: usize,
//     global_array_count: usize,
// }
//
// impl OpenClBlock {
//     pub fn new(vector_size: usize, global_array_count: usize) -> OpenClResult<OpenClBlock> {
//         let device_id = *get_all_devices(CL_DEVICE_TYPE_GPU)?
//             .first()
//             .expect("no device found in platform");
//         let device = Device::new(device_id);
//         println!("{device:?}");
//
//         let context = Context::from_device(&device).expect("Context::from_device failed");
//         println!("{context:?}");
//
//         let queue =
//             CommandQueue::create_default_with_properties(&context, CL_QUEUE_PROFILING_ENABLE, 0)
//                 .expect("CommandQueue::create_default failed");
//         println!("{queue:?}");
//
//         let program_source = gen_vector_program_source(TOTAL_GLOBAL_ARRAY, CAPACITY_GLOBAL_ARRAY, false);
//         // println!("{program_source}");
//         let program =
//             Program::create_and_build_from_source(&context, program_source.as_str(), CL_STD_2_0)
//                 .expect("Program::create_and_build_from_source failed");
//         println!("{program:?}");
//
//         // let vector_add_kernel =
//         //     Kernel::create(&program, KERNEL_NAME).expect("Kernel::create failed");
//         // println!("{vector_add_kernel:?}");
//         //
//         // let vector_extract_kernel =
//         //     Kernel::create(&program, EXTRACT_KERNEL_NAME).expect("Kernel::create failed");
//         // println!("{vector_extract_kernel:?}");
//
//         Ok(OpenClBlock {
//             context,
//             // vector_add_kernel,
//             // vector_extract_kernel,
//             queue,
//             program,
//             global_arrays: HashMap::new(),
//             vector_size,
//             global_array_count,
//         })
//     }
//
//     pub fn create_vector_add_kernel(&self) -> Kernel {
//         let vector_add_kernel =
//             Kernel::create(&self.program, KERNEL_NAME).expect("Kernel::create failed");
//         // println!("{vector_add_kernel:?}");
//         vector_add_kernel
//     }
//
//     pub fn create_vector_extract_kernel(&self) -> Kernel {
//         let vector_extract_kernel =
//             Kernel::create(&self.program, EXTRACT_KERNEL_NAME).expect("Kernel::create failed");
//         // println!("{vector_extract_kernel:?}");
//         vector_extract_kernel
//     }
//
//     pub fn enqueue_buffer(
//         &mut self,
//         vector_add_kernel: &Kernel,
//         buf: &[u8],
//         global_array_index: u32,
//     ) -> OpenClResult<()> {
//         // println!("LIST_SIZE {LIST_SIZE}");
//         // if buf.len() > self.vector_size {
//         //     println!("buffer too large");
//         //     // return Err(ClError(INVALID_BUFFER_LEN));
//         //     return Err(OpenclError::CustomOpenCl(INVALID_BUFFER_LEN));
//         // }
//
//         if global_array_index > self.global_array_count as u32 {
//             println!("global_array_index not valid");
//             return Err(OpenclError::CustomOpenCl(INVALID_GLOBAL_ARRAY_ID));
//         }
//
//         // match self.global_arrays.get(&global_array_index) {
//         //     Some(_) => return Err(OpenclError::CustomOpenCl(GLOBAL_ARRAY_ID_ASSIGNED)),
//         //     None => {
//         //         // pass
//         //     }
//         // }
//
//         let mut input_mem_obj = unsafe {
//             Buffer::<cl_int>::create(
//                 &self.context,
//                 CL_MEM_READ_ONLY,
//                 self.vector_size,
//                 ptr::null_mut(),
//             )?
//         };
//
//         // select global array
//         let d = vec![global_array_index as cl_int; self.vector_size];
//         let mut d_mem_obj = unsafe {
//             Buffer::<cl_int>::create(
//                 &self.context,
//                 CL_MEM_READ_ONLY,
//                 self.vector_size,
//                 ptr::null_mut(),
//             )?
//         };
//         let _d_write_event = unsafe {
//             &self
//                 .queue
//                 .enqueue_write_buffer(&mut d_mem_obj, CL_BLOCKING, 0, &d, &[])?
//         };
//
//         let mut input = vec![-1; self.vector_size];
//
//         if self.vector_size > buf.len() {
//             for (i, v) in buf.iter().enumerate() {
//                 input[i] = *v as cl_int;
//             }
//         } else {
//             for i in 0..self.vector_size {
//                 input[i] = buf[i] as cl_int;
//             }
//         }
//
//         let _write_event = unsafe {
//             &self
//                 .queue
//                 .enqueue_write_buffer(&mut input_mem_obj, CL_BLOCKING, 0, &input, &[])?
//         };
//
//         let event = unsafe {
//             let mut ex = ExecuteKernel::new(vector_add_kernel);
//
//             ex.set_arg(&input_mem_obj).set_arg(&d_mem_obj);
//
//             ex.set_global_work_size(self.vector_size)
//                 .set_local_work_size(64)
//                 .enqueue_nd_range(&self.queue)?
//         };
//
//         event.wait().expect("event.wait");
//
//         self.global_arrays
//             .insert(global_array_index, buf.len() as u64);
//
//         Ok(())
//     }
//
//     pub fn dequeue_buffer(
//         &self,
//         vector_extract_kernel: &Kernel,
//         global_array_index: u32,
//     ) -> OpenClResult<Vec<u8>> {
//         if global_array_index > self.global_array_count as u32 {
//             println!("global_array_index not valid");
//             return Err(OpenclError::CustomOpenCl(INVALID_GLOBAL_ARRAY_ID));
//         }
//
//         let output_mem_obj = unsafe {
//             Buffer::<cl_int>::create(
//                 &self.context,
//                 CL_MEM_WRITE_ONLY,
//                 self.vector_size,
//                 ptr::null_mut(),
//             )?
//         };
//
//         // select global array
//         let d = vec![global_array_index as cl_int; self.vector_size];
//         let mut d_mem_obj = unsafe {
//             Buffer::<cl_int>::create(
//                 &self.context,
//                 CL_MEM_READ_ONLY,
//                 self.vector_size,
//                 ptr::null_mut(),
//             )?
//         };
//         let _d_write_event = unsafe {
//             &self
//                 .queue
//                 .enqueue_write_buffer(&mut d_mem_obj, CL_BLOCKING, 0, &d, &[])?
//         };
//
//         let kernel_event = unsafe {
//             let mut ex = ExecuteKernel::new(vector_extract_kernel);
//
//             ex.set_arg(&output_mem_obj).set_arg(&d_mem_obj);
//
//             ex.set_global_work_size(self.vector_size)
//                 .set_local_work_size(64)
//                 .enqueue_nd_range(&self.queue)?
//         };
//
//         let mut events: Vec<cl_event> = Vec::default();
//         events.push(kernel_event.get());
//
//         let mut output = vec![-1; self.vector_size];
//
//         let _read_event = unsafe {
//             &self.queue.enqueue_read_buffer(
//                 &output_mem_obj,
//                 CL_BLOCKING,
//                 0,
//                 &mut output,
//                 &events,
//             )?
//         };
//
//         // Wait for the read_event to complete.
//         // read_event.wait()?;
//
//         let output_vec: Vec<u8> = output
//             .iter()
//             // .cloned()
//             .filter_map(|x| -> Option<u8> {
//                 if *x > -1 {
//                     return Some(*x as u8);
//                 }
//                 None
//             })
//             // .filter(|&x| *x > -1)
//             // .map(|x| *x as u8)
//             .collect();
//
//         // println!("output {output:?}");
//         // println!(
//         //     "consume arr: {}",
//         //     String::from_utf8(output_vec.clone()).expect("from_utf8")
//         // );
//
//         // println!("output_vec len {}", output_vec.len());
//         // println!("output_vec {:?}", output_vec);
//
//         // Ok(output)
//         Ok(output_vec)
//     }
//
//     pub fn get_global_array_index(&self) -> OpenClResult<u32> {
//         let v = &mut self.global_arrays.keys().collect::<Vec<&u32>>();
//         v.sort();
//         // println!("get_global_array_index {v:?}");
//         match v.pop() {
//             Some(i) => {
//                 if *i > self.global_array_count as u32 {
//                     return Err(OpenclError::CustomOpenCl(NO_GLOBAL_VECTORS_TO_ASSIGN));
//                 }
//                 Ok(i + 1)
//             }
//             None => Ok(0),
//         }
//     }
//
//     pub fn assign_global_array_index(&mut self, kernel: Option<Kernel>) -> OpenClResult<u32> {
//         let i = self.get_global_array_index()?;
//         // if program opencl initialize with -1
//         // self.global_arrays.insert(i, 0);
//
//         // if program opencl initialize with 0
//         // fill opencl array with -1, from a kernel call
//         let k = match kernel {
//             None => self.create_vector_add_kernel(),
//             Some(v) => v,
//         };
//         self.enqueue_buffer(&k, &[], i)?;
//         Ok(i)
//     }
//
//     pub fn show_global_arrays(&self) {
//         println!("{:#?}", self.global_arrays);
//     }
//
//     pub fn get_global_arrays(&self) -> &GlobalArrayMap {
//         &self.global_arrays
//     }
// }

const TEMPLATE_KERNEL_VECTOR_ADD: &str = r#"
    kernel void KERNEL_NAME(global int* A, global int* D) {

        // Get the index of the current element
        int i = get_global_id(0);

        // Do the operation
        switch (D[i]) {
            SWITCH_BODY
        }
    }
    "#;

const TEMPLATE_KERNEL_VECTOR_EXTRACT: &str = r#"
    kernel void KERNEL_NAME(global int* C, global int* D) {

        // Get the index of the current element
        int i = get_global_id(0);

        switch (D[i]) {
            SWITCH_BODY
        }
    }
    "#;

pub struct GlobalArrayBlockConfig {
    name: &'static str,
    global_array_capacity: usize,
    global_array_count: usize,
    negative_initial_value: bool
}

pub fn gen_vector_program_source(block_config: Vec<GlobalArrayBlockConfig>) -> String {
    let mut global_arrays = String::from("");
    let mut vector_add_kernel_list = String::from("");
    let mut vector_extract_kernel_list = String::from("");

    for config in block_config {
        let block_name = config.name;
        let mut vector_add_switch = String::from("");
        let mut vector_extract_switch = String::from("");

        for i in 0..config.global_array_count {
            let capacity = config.global_array_capacity;
            let arr_name = format!("arr_{block_name}_{i}");

            // global arrays
            let global_arr = if config.negative_initial_value {
                // slow compilation
                let initialize =
                    "= { [0 ... limit] = -1 };".replace("limit", (capacity - 1).to_string().as_str());
                format!("
    __global int {arr_name}[{capacity}] {initialize}"
                )
            } else {
                format!("
    __global int {arr_name}[{capacity}];"
                )
            };
            global_arrays.push_str(&global_arr);

            // vector_add_kernel
            let v_add = format!("
                case {i}:
                  {arr_name}[i] = A[i];
                  break;"
            );
            vector_add_switch.push_str(&v_add);

            // vector_extract
            let v_ext = format!("
            case {i}:
              C[i] = {arr_name}[i];
              break;"
            );
            vector_extract_switch.push_str(&v_ext);
        }

        // add space
        global_arrays.push_str("
        ");

        let kernel_fn = TEMPLATE_KERNEL_VECTOR_ADD
            .replace("KERNEL_NAME", format!("{VECTOR_ADD_KERNEL_NAME}_{block_name}").as_str())
            .replace("SWITCH_BODY", &vector_add_switch);
        vector_add_kernel_list.push_str(&kernel_fn);

        let kernel_fn = TEMPLATE_KERNEL_VECTOR_EXTRACT
            .replace("KERNEL_NAME", format!("{VECTOR_EXTRACT_KERNEL_NAME}_{block_name}").as_str())
            .replace("SWITCH_BODY", &vector_extract_switch);
        vector_extract_kernel_list.push_str(&kernel_fn);

    }

    // TEMPLATE_SOURCE
    //     .replace("__GLOBAL_ARRAYS", &global_arrays)
        // .replace("VECTOR_ADD_SWITCH", &vector_add_switch)
        // .replace("VECTOR_EXTRACT_SWITCH", &vector_extract_switch)
    format!("
    {global_arrays}
    {vector_add_kernel_list}
    {vector_extract_kernel_list}
    ")
}

// pub fn load_dirs(dir: &Path, vec: &mut Vec<DirEntry>) -> io::Result<()> {
//     if dir.is_dir() {
//         for entry in fs::read_dir(dir)? {
//             let entry = entry?;
//             let path = entry.path();
//             if path.is_dir() {
//                 load_dirs(&path, vec)?;
//             } else {
//                 let size = entry.metadata().unwrap().len() as f64;
//                 let size_mb = size / (1024 * 1024) as f64;
//
//                 let path = entry.path();
//                 let path_str = path.to_str().unwrap();
//
//                 // println!("{}", path_str);
//                 // println!("size {size} bytes -> {} mb", size_mb);
//
//                 // if size_mb > 0.9 {
//                 //     println!("{}", path_str);
//                 //     println!("size {size} bytes -> {} mb", size_mb);
//                 //     vec.push(entry);
//                 // }
//                 if size < 1024 as f64 {
//                     println!("{}", path_str);
//                     println!("size {size} bytes -> {} mb", size_mb);
//                     vec.push(entry);
//                 }
//             }
//         }
//     }
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_program_source() {
        let v: Vec<GlobalArrayBlockConfig> = vec![
            GlobalArrayBlockConfig {
                name: "1kb",
                global_array_capacity: KB_1,
                global_array_count: 3,
                negative_initial_value: false,
            },
            GlobalArrayBlockConfig {
                name: "1mb",
                global_array_capacity: MB_1,
                global_array_count: 2,
                negative_initial_value: false,
            }
        ];
        let result = gen_vector_program_source(v);
        println!("{result}");
        assert_ne!(result.len(), 0);
    }
}
