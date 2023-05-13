use crate::error::OpenClResult;
use crate::ocl_v5::{OpenClBlock, LIST_SIZE, TOTAL_GLOBAL_ARRAY};
use io::Error as IoError;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
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
    println!("path total {}", ocl_fs.ocl_block.get_global_arrays().len());
    println!("index total {}", ocl_fs.ocl_block.get_global_arrays().len());
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
        OclFile::create(path)?.write_all(contents)
    }
    inner(path.as_ref(), contents.as_ref())
}

#[derive(Debug, Clone)]
pub struct OclFile {
    global_array_index: u32,
}

impl OclFile {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<OclFile> {
        let ocl_fs = GLOBAL_OCL_FS.lock().unwrap();

        // TODO open flags
        let path_b = path.as_ref().as_os_str().as_bytes();

        match ocl_fs.cache.get(String::from_utf8_lossy(path_b).as_ref()) {
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

    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<OclFile> {
        let mut ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
        let path_str = String::from_utf8_lossy(path.as_ref().as_os_str().as_bytes());

        let index = match ocl_fs.cache.get(path_str.as_ref()) {
            None => ocl_fs.ocl_block.assign_global_array_index(None)?,
            Some(v) => *v,
        };
        ocl_fs.cache.insert(path_str.into(), index);

        Ok(OclFile {
            global_array_index: index,
        })
    }

    pub fn global_array_index(&self) -> u32 {
        self.global_array_index
    }

    fn read_to_vec(&self) -> io::Result<Vec<u8>> {
        let ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
        let k = ocl_fs.ocl_block.create_vector_extract_kernel();
        let v = ocl_fs
            .ocl_block
            .dequeue_buffer(&k, self.global_array_index)?;
        Ok(v)
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
