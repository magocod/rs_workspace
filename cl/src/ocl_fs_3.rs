use crate::error::OpenClResult;
use crate::ocl_v7::{default_memory_block, explain_memory_block_config, MemoryBlock, OpenClBlock};
use io::Error as IoError;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref GLOBAL_OCL_FS: Mutex<OclFs> = Mutex::new(OclFs::new());
}

lazy_static! {
    static ref GLOBAL_MEMORY_BLOCK: Mutex<MemoryBlock> = Mutex::new(default_memory_block());
}

lazy_static! {
    static ref GLOBAL_OPENCL_BLOCK: Arc<OpenClBlock> =
        Arc::new(OpenClBlock::new(default_memory_block()).expect("OpenClBlock::new()"));
}

pub fn ocl_initialize() {
    let ocl_block = GLOBAL_OPENCL_BLOCK.clone();
    ocl_block.initialize_kernel();
}

pub fn ocl_cache() -> OpenClResult<()> {
    let ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
    let memory_block = GLOBAL_MEMORY_BLOCK.lock().unwrap();
    // println!("{:#?}", ocl_fs.cache);
    // println!("{:#?}", memory_block.memory_map());
    println!("path total {}", ocl_fs.cache.len());
    println!("index total {}", memory_block.memory_map().len());

    let (v, t, c) = explain_memory_block_config(memory_block.config_map());
    println!("{v:#?}");
    println!("{t} mb, blocks: {c}");
    Ok(())
}

pub fn ocl_summary() -> OpenClResult<()> {
    let memory_block = GLOBAL_MEMORY_BLOCK.lock().unwrap();
    let v = memory_block.summary();
    println!("{:#?}", v);

    let t = v.iter().map(|x| x.assigned).collect::<Vec<u64>>();
    let r = t.into_iter().reduce(|acc, x| acc + x).unwrap();
    println!("assigned {r}");
    Ok(())
}

pub fn ocl_cache_map() -> FileCacheMap {
    let ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
    ocl_fs.cache.clone()
}

pub type FileCacheMap = HashMap<String, String>;

#[derive(Debug)]
struct OclFs {
    pub cache: FileCacheMap,
}

impl OclFs {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
}

pub fn read<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    fn inner(path: &Path) -> io::Result<Vec<u8>> {
        let file = OclFile::open(path)?;
        // TODO file metadata
        Ok(file.read_to_vec()?)
    }
    inner(path.as_ref())
}

pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fn inner(path: &Path) -> io::Result<String> {
        let file = OclFile::open(path)?;
        // TODO file metadata
        let st = String::from_utf8(file.read_to_vec()?).unwrap();
        Ok(st)
    }
    inner(path.as_ref())
}

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    fn inner(path: &Path, contents: &[u8]) -> io::Result<()> {
        OclFile::create_with_len(path, contents.len())?.write_all(contents)
    }
    inner(path.as_ref(), contents.as_ref())
}

#[derive(Debug, Clone)]
pub struct OclFile {
    key: String,
}

impl OclFile {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<OclFile> {
        // TODO open flags
        let path_b = path.as_ref().as_os_str().to_string_lossy();

        let ocl_fs = GLOBAL_OCL_FS.lock().unwrap();

        match ocl_fs.cache.get(path_b.as_ref()) {
            Some(v) => {
                let memory_block = GLOBAL_MEMORY_BLOCK.lock().unwrap();
                if memory_block.memory_map().get(v).is_some() {
                    return Ok(OclFile { key: v.clone() });
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

    pub fn create<P: AsRef<Path>>(_: P) -> io::Result<OclFile> {
        unimplemented!();
    }

    pub fn create_with_len<P: AsRef<Path>>(path: P, len: usize) -> io::Result<OclFile> {
        let path_b = path.as_ref().as_os_str().to_string_lossy();

        let mut memory_block = GLOBAL_MEMORY_BLOCK.lock().unwrap();
        let key = memory_block.set_key_by_len(len)?;

        let mut ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
        ocl_fs.cache.insert(path_b.into(), key.clone());

        Ok(OclFile { key })
    }

    pub fn key(&self) -> &String {
        &self.key
    }

    fn read_to_vec(&self) -> io::Result<Vec<u8>> {
        let ocl_block = GLOBAL_OPENCL_BLOCK.clone();
        let v = ocl_block.dequeue_buffer(&self.key)?;
        Ok(v)
    }
}

impl Write for OclFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let ocl_block = GLOBAL_OPENCL_BLOCK.clone();
        let r = ocl_block.enqueue_buffer(buf, &self.key)?;
        if r >= buf.len() {
            return Ok(buf.len());
        }
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for OclFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let ocl_block = GLOBAL_OPENCL_BLOCK.clone();
        let v = ocl_block.dequeue_buffer(&self.key)?;

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
