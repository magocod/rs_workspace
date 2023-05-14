use crate::error::{OpenClResult, OpenclError, INVALID_BUFFER_LEN, INVALID_GLOBAL_ARRAY_ID, NO_GLOBAL_VECTORS_TO_ASSIGN, INVALID_KERNEL_BLOCK_NAME};
use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
use opencl3::program::{Program, CL_STD_2_0};
use opencl3::types::{cl_event, cl_int, CL_BLOCKING};
use std::collections::HashMap;
use std::ptr;

pub const KB_1: usize = 1024;
pub const KB_1_S: &str = "1kb";

pub const MB_1: usize = KB_1 * KB_1;
pub const MB_1_S: &str = "1mb";

pub const SIZE: usize = KB_1 * 1;

pub const LIST_SIZE: usize = SIZE;
pub const TOTAL_GLOBAL_ARRAY: usize = 128;
pub const CAPACITY_GLOBAL_ARRAY: usize = SIZE;

pub const VECTOR_ADD_KERNEL_NAME: &str = "vector_add";
pub const VECTOR_EXTRACT_KERNEL_NAME: &str = "vector_extract";

#[derive(Debug, Clone)]
pub struct GlobalArrayBlockConfig {
    global_array_capacity: usize,
    global_array_count: usize,
    negative_initial_value: bool,
}

pub type BlockConfigMap = HashMap<&'static str, GlobalArrayBlockConfig>;

pub fn default_config() -> BlockConfigMap {
    let mut m = HashMap::new();
    m.insert(
        KB_1_S,
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1,
            global_array_count: 2500,
            negative_initial_value: false,
        },
    );
    m.insert(
        MB_1_S,
        GlobalArrayBlockConfig {
            global_array_capacity: MB_1,
            global_array_count: 2,
            negative_initial_value: false,
        },
    );
    m
}

#[derive(Debug, Clone)]
pub struct KernelBlockSize<'a> {
    block_name: &'a str,
    size: u64,
}

// discarded
// pub type KernelBlockMap = HashMap<u32, u64>;
// pub type GlobalArrayMap = HashMap<&'static str, KernelBlockMap>;

pub type GlobalArrayMap = HashMap<String, u64>;

#[derive(Debug)]
pub struct OpenClBlock {
    context: Context,
    queue: CommandQueue,
    program: Program,
    global_array_map: GlobalArrayMap,
    config: BlockConfigMap,
}

pub fn get_kernel_name(kernel_name: &str, block_name: &str) -> String {
    format!("{kernel_name}_{block_name}")
}

/// last element in string separated with (_), is the index
pub fn get_global_array_index(string_key: &str) -> u64 {
    let index = string_key.split("_").collect::<Vec<&str>>().pop().unwrap();
    index.parse::<u64>().unwrap()
}

pub fn get_global_array_block(string_key: &str) -> &str {
    let block_name = *string_key
        .split("_")
        .collect::<Vec<&str>>()
        .first()
        .unwrap();
    block_name
}

impl<'a> OpenClBlock {
    pub fn new(config: BlockConfigMap) -> OpenClResult<OpenClBlock> {
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

        let program_source = gen_vector_program_source(&config);
        // println!("{program_source}");
        let program =
            Program::create_and_build_from_source(&context, program_source.as_str(), CL_STD_2_0)
                .expect("Program::create_and_build_from_source failed");
        println!("{program:?}");

        // discarded
        // let mut global_array_map = HashMap::new();
        // for (k, v) in config.iter() {
        //     global_array_map.insert(*k, HashMap::new());
        // }

        Ok(OpenClBlock {
            context,
            queue,
            program,
            global_array_map: HashMap::new(),
            config,
        })
    }

    pub fn initialize_kernel(&self) {
        for (k, _) in &self.config {
            self.create_vector_add_kernel(*k);
            self.create_vector_extract_kernel(*k);
        }
    }

    fn create_kernel(&self, kernel_name: &str, block_name: &str) -> Kernel {
        let name = get_kernel_name(kernel_name, block_name);
        Kernel::create(&self.program, &name)
            .expect(format!("Kernel::create {} failed", &name).as_str())
    }

    pub fn create_vector_add_kernel(&self, block_name: &str) -> Kernel {
        self.create_kernel(VECTOR_ADD_KERNEL_NAME, block_name)
    }

    pub fn create_vector_extract_kernel(&self, block_name: &str) -> Kernel {
        self.create_kernel(VECTOR_EXTRACT_KERNEL_NAME, block_name)
    }

    pub fn enqueue_buffer(&mut self, buf: &[u8], global_array_key: &str) -> OpenClResult<()> {
        let (block_name, block_config) = self.get_block_config_by_key(global_array_key)?;
        let vector_size = block_config.global_array_capacity;
        let global_array_index = get_global_array_index(global_array_key);

        if global_array_index > (block_config.global_array_count - 1) as u64 {
            println!("global_array_index not valid, {global_array_index}, {}", buf.len());
            return Err(OpenclError::CustomOpenCl(INVALID_GLOBAL_ARRAY_ID));
        }

        let mut input_mem_obj = unsafe {
            Buffer::<cl_int>::create(
                &self.context,
                CL_MEM_READ_ONLY,
                vector_size,
                ptr::null_mut(),
            )?
        };

        // TODO opencl kernel update arg index
        // select global array
        let d = vec![global_array_index as cl_int; vector_size];
        let mut d_mem_obj = unsafe {
            Buffer::<cl_int>::create(
                &self.context,
                CL_MEM_READ_ONLY,
                vector_size,
                ptr::null_mut(),
            )?
        };
        let _d_write_event = unsafe {
            &self
                .queue
                .enqueue_write_buffer(&mut d_mem_obj, CL_BLOCKING, 0, &d, &[])?
        };

        // TODO update vector assignment
        let mut input = vec![-1; vector_size];

        if vector_size > buf.len() {
            for (i, v) in buf.iter().enumerate() {
                input[i] = *v as cl_int;
            }
        } else {
            for i in 0..vector_size {
                input[i] = buf[i] as cl_int;
            }
        }

        let _write_event = unsafe {
            &self
                .queue
                .enqueue_write_buffer(&mut input_mem_obj, CL_BLOCKING, 0, &input, &[])?
        };

        let kernel = self.create_vector_add_kernel(block_name);

        let event = unsafe {
            ExecuteKernel::new(&kernel)
                .set_arg(&input_mem_obj)
                .set_arg(&d_mem_obj)
                .set_global_work_size(vector_size)
                .set_local_work_size(64)
                .enqueue_nd_range(&self.queue)?
        };

        event.wait().expect("event.wait");

        self.global_array_map
            .insert(global_array_key.into(), buf.len() as u64);

        Ok(())
    }

    pub fn dequeue_buffer(&self, global_array_key: &str) -> OpenClResult<Vec<u8>> {
        let (block_name, block_config) = self.get_block_config_by_key(global_array_key)?;
        let vector_size = block_config.global_array_capacity;
        let global_array_index = get_global_array_index(global_array_key);

        if global_array_index > block_config.global_array_count as u64 {
            println!("global_array_index not valid, {global_array_index}");
            return Err(OpenclError::CustomOpenCl(INVALID_GLOBAL_ARRAY_ID));
        }

        let output_mem_obj = unsafe {
            Buffer::<cl_int>::create(
                &self.context,
                CL_MEM_WRITE_ONLY,
                vector_size,
                ptr::null_mut(),
            )?
        };

        // select global array
        let d = vec![global_array_index as cl_int; vector_size];
        let mut d_mem_obj = unsafe {
            Buffer::<cl_int>::create(
                &self.context,
                CL_MEM_READ_ONLY,
                vector_size,
                ptr::null_mut(),
            )?
        };
        let _d_write_event = unsafe {
            &self
                .queue
                .enqueue_write_buffer(&mut d_mem_obj, CL_BLOCKING, 0, &d, &[])?
        };

        let kernel = self.create_vector_extract_kernel(block_name);

        let kernel_event = unsafe {
            ExecuteKernel::new(&kernel)
                .set_arg(&output_mem_obj)
                .set_arg(&d_mem_obj)
                .set_global_work_size(vector_size)
                .set_local_work_size(64)
                .enqueue_nd_range(&self.queue)?
        };

        let mut events: Vec<cl_event> = Vec::default();
        events.push(kernel_event.get());

        let mut output = vec![-1; vector_size];

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

        let output_vec: Vec<u8> = output
            .iter()
            // .cloned()
            .filter_map(|x| -> Option<u8> {
                if *x > -1 {
                    return Some(*x as u8);
                }
                None
            })
            // .filter(|&x| *x > -1)
            // .map(|x| *x as u8)
            .collect();

        Ok(output_vec)
    }

    pub fn get_block_config_by_len(
        &self,
        len: usize,
    ) -> OpenClResult<(&str, &GlobalArrayBlockConfig)> {
        // search block
        let mut v: Vec<KernelBlockSize> = self
            .config
            .iter()
            .filter_map(|(&block_name, config)| -> Option<KernelBlockSize> {
                // check assigned index
                let assigned = self
                    .global_array_map
                    .iter()
                    .filter_map(|(index_key, _)| -> Option<u64> {
                        if index_key.contains(block_name) {
                            return Some(get_global_array_index(index_key));
                        }
                        None
                    })
                    .collect::<Vec<u64>>().len();

                if assigned >= config.global_array_count {
                    return None
                }

                Some(KernelBlockSize {
                    block_name,
                    size: config.global_array_capacity as u64,
                })
            })
            .collect();
        v.sort_by(|a, b| a.size.cmp(&b.size));

        let block_size = v
            .iter()
            .find(|x| x.size >= len as u64)
            .ok_or(OpenclError::CustomOpenCl(INVALID_BUFFER_LEN))?;

        // return block configuration
        match self.config.get(block_size.block_name) {
            None => Err(OpenclError::CustomOpenCl(INVALID_KERNEL_BLOCK_NAME)),
            Some(v) => Ok((block_size.block_name, v)),
        }
    }

    pub fn get_block_config_by_key(
        &self,
        key: &str,
    ) -> OpenClResult<(&str, &GlobalArrayBlockConfig)> {
        // search block
        let block_name = get_global_array_block(key);

        // return block configuration
        for (&k, v) in self.config.iter() {
            if k == block_name {
                return Ok((k, v));
            }
        }
        Err(OpenclError::CustomOpenCl(INVALID_GLOBAL_ARRAY_ID))
    }

    pub fn get_block_key(&self, len: usize) -> OpenClResult<String> {
        let (block_name, block_config) = self.get_block_config_by_len(len)?;

        let mut v: Vec<u64> = self
            .global_array_map
            .iter()
            .filter_map(|(k, _)| -> Option<u64> {
                if k.contains(block_name) {
                    return Some(get_global_array_index(k));
                }
                None
            })
            .collect();
        v.sort();

        match v.pop() {
            Some(i) => {
                if i >= (block_config.global_array_count - 1) as u64 {
                    return Err(OpenclError::CustomOpenCl(NO_GLOBAL_VECTORS_TO_ASSIGN));
                }
                Ok(format!("{block_name}_{}", i + 1))
            }
            None => Ok(format!("{block_name}_0")),
        }
    }

    pub fn assign_global_array_key(&mut self, len: usize) -> OpenClResult<String> {
        let k = self.get_block_key(len)?;
        // if program opencl initialize with 0
        // fill opencl array with -1, from a kernel call
        self.enqueue_buffer(&[], &k)?;

        // if program opencl initialize with -1
        // self.global_array_map.insert(k.clone(), 0);
        Ok(k)
    }

    pub fn get_global_array_map(&self) -> &GlobalArrayMap {
        &self.global_array_map
    }

    #[cfg(test)]
    pub fn fake_block_allocation(&mut self, k: &str) {
        self.global_array_map.insert(k.to_string(), 0);
    }
}

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

pub fn gen_vector_program_source(block_config: &BlockConfigMap) -> String {
    let mut global_arrays = String::from("");
    let mut vector_add_kernel_list = String::from("");
    let mut vector_extract_kernel_list = String::from("");

    for (k, config) in block_config {
        let block_name = *k;
        let mut vector_add_switch = String::from("");
        let mut vector_extract_switch = String::from("");

        for i in 0..config.global_array_count {
            let capacity = config.global_array_capacity;
            let arr_name = format!("arr_{block_name}_{i}");

            // global arrays
            let global_arr = if config.negative_initial_value {
                // slow compilation
                let initialize = "= { [0 ... limit] = -1 };"
                    .replace("limit", (capacity - 1).to_string().as_str());
                format!(
                    "
    __global int {arr_name}[{capacity}] {initialize}"
                )
            } else {
                format!(
                    "
    __global int {arr_name}[{capacity}];"
                )
            };
            global_arrays.push_str(&global_arr);

            // vector_add_kernel
            let v_add = format!(
                "
                case {i}:
                  {arr_name}[i] = A[i];
                  break;"
            );
            vector_add_switch.push_str(&v_add);

            // vector_extract
            let v_ext = format!(
                "
            case {i}:
              C[i] = {arr_name}[i];
              break;"
            );
            vector_extract_switch.push_str(&v_ext);
        }

        // add space
        global_arrays.push_str(
            "
        ",
        );

        let kernel_fn = TEMPLATE_KERNEL_VECTOR_ADD
            .replace(
                "KERNEL_NAME",
                format!("{VECTOR_ADD_KERNEL_NAME}_{block_name}").as_str(),
            )
            .replace("SWITCH_BODY", &vector_add_switch);
        vector_add_kernel_list.push_str(&kernel_fn);

        let kernel_fn = TEMPLATE_KERNEL_VECTOR_EXTRACT
            .replace(
                "KERNEL_NAME",
                format!("{VECTOR_EXTRACT_KERNEL_NAME}_{block_name}").as_str(),
            )
            .replace("SWITCH_BODY", &vector_extract_switch);
        vector_extract_kernel_list.push_str(&kernel_fn);
    }

    format!(
        "
    {global_arrays}
    {vector_add_kernel_list}
    {vector_extract_kernel_list}
    "
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_program_source() {
        let mut m: BlockConfigMap = HashMap::new();
        m.insert(
            "1kb",
            GlobalArrayBlockConfig {
                global_array_capacity: KB_1,
                global_array_count: 3,
                negative_initial_value: false,
            },
        );
        m.insert(
            "1mb",
            GlobalArrayBlockConfig {
                global_array_capacity: MB_1,
                global_array_count: 2,
                negative_initial_value: false,
            },
        );

        let result = gen_vector_program_source(&m);
        println!("{result}");
        assert_ne!(result.len(), 0);
    }

    #[test]
    fn get_block_name() {
        let c = default_config();
        let ocl_block = OpenClBlock::new(c).unwrap();

        let o1 = ocl_block.get_block_config_by_len(KB_1).ok().unwrap();
        let o2 = ocl_block.get_block_config_by_len(MB_1).ok().unwrap();
        let o4 = ocl_block.get_block_config_by_len(KB_1 + 1).ok().unwrap();
        let o5 = ocl_block.get_block_config_by_len(KB_1 - 1).ok().unwrap();
        let o6 = ocl_block.get_block_config_by_len(0).ok().unwrap();

        let o3 = ocl_block.get_block_config_by_len(MB_1 * 4).is_err();

        assert_eq!(o1.0, KB_1_S);
        assert_eq!(o2.0, MB_1_S);
        assert_eq!(o4.0, MB_1_S);
        assert_eq!(o5.0, KB_1_S);
        assert_eq!(o6.0, KB_1_S);

        assert_eq!(o3, true);
    }

    #[test]
    fn get_block_key_1() {
        let mut c: BlockConfigMap = HashMap::new();
        c.insert(
            "1kb",
            GlobalArrayBlockConfig {
                global_array_capacity: KB_1,
                global_array_count: 3,
                negative_initial_value: false,
            },
        );
        let mut ocl_block = OpenClBlock::new(c).unwrap();

        let r1 = ocl_block.get_block_key(KB_1).unwrap();
        println!("r1 {r1:?}");
        ocl_block.fake_block_allocation(&r1);
        let r2 = ocl_block.get_block_key(KB_1 - 256).unwrap();
        println!("r2 {r2:?}");
        ocl_block.fake_block_allocation(&r2);
        let r3 = ocl_block.get_block_key(KB_1 - 512).unwrap();
        println!("r3 {r3:?}");
        ocl_block.fake_block_allocation(&r3);

        let r4 = ocl_block.get_block_key(KB_1).is_err();
        println!("r4 {r4:?}");

        println!("cache {:#?}", ocl_block.get_global_array_map());

        assert_eq!(r1, "1kb_0");
        assert_eq!(r2, "1kb_1");
        assert_eq!(r3, "1kb_2");
        assert_eq!(r4, true);
        assert_eq!(ocl_block.get_global_array_map().len(), 3);
    }

    #[test]
    fn get_block_key_2() {
        let mut c: BlockConfigMap = HashMap::new();
        c.insert(
            "1kb",
            GlobalArrayBlockConfig {
                global_array_capacity: KB_1,
                global_array_count: 1,
                negative_initial_value: false,
            },
        );
        let mut ocl_block = OpenClBlock::new(c).unwrap();

        let r1 = ocl_block.get_block_key(KB_1).unwrap();
        println!("r1 {r1:?}");
        ocl_block.fake_block_allocation(&r1);
        let r2 = ocl_block.get_block_key(KB_1).is_err();
        println!("r2 {r2:?}");

        println!("cache {:#?}", ocl_block.get_global_array_map());

        assert_eq!(r1, "1kb_0");
        assert_eq!(r2, true);
        assert_eq!(ocl_block.get_global_array_map().len(), 1);
    }

    #[test]
    fn get_block_key_3() {
        let mut c: BlockConfigMap = HashMap::new();
        c.insert(
            "1kb",
            GlobalArrayBlockConfig {
                global_array_capacity: KB_1,
                global_array_count: 0,
                negative_initial_value: false,
            },
        );
        let ocl_block = OpenClBlock::new(c).unwrap();
        let r = ocl_block.get_block_key(KB_1).is_err();
        assert_eq!(r, true);
    }

    #[test]
    fn get_block_key_4() {
        let mut c: BlockConfigMap = HashMap::new();
        c.insert(
            "1kb",
            GlobalArrayBlockConfig {
                global_array_capacity: KB_1,
                global_array_count: 1,
                negative_initial_value: false,
            },
        );
        c.insert(
            "1mb",
            GlobalArrayBlockConfig {
                global_array_capacity: MB_1,
                global_array_count: 1,
                negative_initial_value: false,
            },
        );
        let mut ocl_block = OpenClBlock::new(c).unwrap();

        let r1 = ocl_block.get_block_key(KB_1).unwrap();
        println!("r1 {r1:?}");
        ocl_block.fake_block_allocation(&r1);
        let r2 = ocl_block.get_block_key(KB_1).unwrap();
        println!("r2 {r2:?}");
        ocl_block.fake_block_allocation(&r2);
        let r3 = ocl_block.get_block_key(KB_1).is_err();
        println!("r3 {r3:?}");

        println!("cache {:#?}", ocl_block.get_global_array_map());

        assert_eq!(r1, "1kb_0");
        assert_eq!(r2, "1mb_0");
        assert_eq!(r3, true);
        assert_eq!(ocl_block.get_global_array_map().len(), 2);
    }

    #[test]
    fn get_block_key_5() {
        let mut c: BlockConfigMap = HashMap::new();
        c.insert(
            "1kb",
            GlobalArrayBlockConfig {
                global_array_capacity: KB_1,
                global_array_count: 1,
                negative_initial_value: false,
            },
        );
        c.insert(
            "0cap",
            GlobalArrayBlockConfig {
                global_array_capacity: KB_1,
                global_array_count: 0,
                negative_initial_value: false,
            },
        );
        c.insert(
            "512b",
            GlobalArrayBlockConfig {
                global_array_capacity: KB_1 - 512,
                global_array_count: 1,
                negative_initial_value: false,
            },
        );
        c.insert(
            "1mb",
            GlobalArrayBlockConfig {
                global_array_capacity: MB_1,
                global_array_count: 1,
                negative_initial_value: false,
            },
        );
        let mut ocl_block = OpenClBlock::new(c).unwrap();

        let r1 = ocl_block.get_block_key(MB_1 * 4).is_err();
        println!("r1 {r1:?}");

        let r2 = ocl_block.get_block_key(MB_1).unwrap();
        println!("r2 {r2:?}");
        ocl_block.fake_block_allocation(&r2);

        let r3 = ocl_block.get_block_key(KB_1 - 256).unwrap();
        println!("r3 {r3:?}");
        ocl_block.fake_block_allocation(&r3);

        let r4 = ocl_block.get_block_key(KB_1 - 512 - 128).unwrap();
        println!("r4 {r4:?}");
        ocl_block.fake_block_allocation(&r4);

        let r5 = ocl_block.get_block_key(KB_1).is_err();
        println!("r5 {r5:?}");

        println!("cache {:#?}", ocl_block.get_global_array_map());

        assert_eq!(r1, true);
        assert_eq!(r2, "1mb_0");
        assert_eq!(r3, "1kb_0");
        assert_eq!(r4, "512b_0");
        assert_eq!(r5, true);
        assert_eq!(ocl_block.get_global_array_map().len(), 3);
    }
}
