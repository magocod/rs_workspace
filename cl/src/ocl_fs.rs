use crate::ocl_v5::{OpenClBlock, LIST_SIZE, TOTAL_GLOBAL_ARRAY};
use io::Error as IoError;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
// use std::path::Path;
use crate::error::OpenClResult;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GLOBAL_OCL_FS: Mutex<OclFs> = Mutex::new(OclFs::new(LIST_SIZE, TOTAL_GLOBAL_ARRAY));
}

pub fn ocl_initialize(with_kernel: bool) {
    let ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
    if with_kernel {
        let _ = ocl_fs.ocl_block.create_vector_add_kernel();
        let _ = ocl_fs.ocl_block.create_vector_extract_kernel();
    }
}

pub fn ocl_cache() -> OpenClResult<()> {
    let ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
    println!("{:#?}", ocl_fs.cache);
    println!("{:#?}", ocl_fs.ocl_block.get_global_arrays());
    Ok(())
}

pub type FileCacheMap = HashMap<String, u32>;

#[derive(Debug)]
struct OclFs {
    pub ocl_block: OpenClBlock,
    pub cache: FileCacheMap,
}

impl OclFs {
    pub fn new(vector_size: usize, global_array_count: usize) -> Self {
        Self {
            ocl_block: OpenClBlock::new(vector_size, global_array_count)
                .expect("OpenClBlock::new()"),
            cache: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OclFile {
    global_array_index: u32,
}

impl OclFile {
    pub fn open(path: String) -> io::Result<OclFile> {
        let ocl_fs = GLOBAL_OCL_FS.lock().unwrap();

        // TODO open flags
        match ocl_fs.cache.get(path.as_str()) {
            Some(v) => {
                if ocl_fs.ocl_block.get_global_arrays().get(v).is_some() {
                    return Ok(OclFile {
                        global_array_index: *v,
                    });
                }
            }
            None => {
                // pass
            }
        }
        Err(IoError::new(
            io::ErrorKind::NotFound,
            "No such file or directory",
        ))
    }

    pub fn create(path: String) -> io::Result<OclFile> {
        let mut ocl_fs = GLOBAL_OCL_FS.lock().unwrap();

        let index = match ocl_fs.cache.get(path.as_str()) {
            None => ocl_fs.ocl_block.assign_global_array_index(None)?,
            Some(v) => *v,
        };
        ocl_fs.cache.insert(path, index);

        Ok(OclFile {
            global_array_index: index,
        })
    }

    pub fn global_array_index(&self) -> u32 {
        self.global_array_index
    }
}

impl Write for OclFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
        let k = ocl_fs.ocl_block.create_vector_add_kernel();
        ocl_fs
            .ocl_block
            .enqueue_buffer(&k, buf, self.global_array_index)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for OclFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
        let k = ocl_fs.ocl_block.create_vector_extract_kernel();
        let v = ocl_fs
            .ocl_block
            .dequeue_buffer(&k, self.global_array_index)?;

        // FIXME Unsafe buffer update
        if buf.len() > v.len() {
            for (i, _) in v.iter().enumerate() {
                buf[i] = v[i];
            }
        } else {
            for i in 0..buf.len() {
                buf[i] = v[i];
            }
        }
        Ok(v.len())
    }
}
