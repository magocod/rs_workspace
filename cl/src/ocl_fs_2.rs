use crate::error::OpenClResult;
use crate::ocl_v6::{default_config, BlockConfigMap, OpenClBlock};
use io::Error as IoError;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    static ref GLOBAL_OCL_FS: Mutex<OclFs> = Mutex::new(OclFs::new(default_config()));
}

pub fn ocl_initialize() {
    let ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
    ocl_fs.ocl_block.initialize_kernel();
}

pub fn ocl_cache() -> OpenClResult<()> {
    let ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
    println!("{:#?}", ocl_fs.cache);
    println!("{:#?}", ocl_fs.ocl_block.get_global_array_map());
    println!("path total {}", ocl_fs.cache.len());
    println!(
        "index total {}",
        ocl_fs.ocl_block.get_global_array_map().len()
    );
    Ok(())
}

pub type FileCacheMap = HashMap<String, String>;

#[derive(Debug)]
struct OclFs {
    pub ocl_block: OpenClBlock,
    pub cache: FileCacheMap,
}

impl OclFs {
    pub fn new(config: BlockConfigMap) -> Self {
        Self {
            ocl_block: OpenClBlock::new(config).expect("OpenClBlock::new()"),
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
    global_array_key: String,
}

impl OclFile {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<OclFile> {
        let ocl_fs = GLOBAL_OCL_FS.lock().unwrap();

        // TODO open flags
        let path_b = path.as_ref().as_os_str().to_string_lossy();

        match ocl_fs.cache.get(path_b.as_ref()) {
            Some(v) => {
                if ocl_fs.ocl_block.get_global_array_map().get(v).is_some() {
                    return Ok(OclFile {
                        global_array_key: v.clone(),
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

    pub fn create<P: AsRef<Path>>(_: P) -> io::Result<OclFile> {
        unimplemented!();
    }

    pub fn create_with_len<P: AsRef<Path>>(path: P, len: usize) -> io::Result<OclFile> {
        let mut ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
        let k = ocl_fs.ocl_block.assign_global_array_key(len)?;
        let path_b = path.as_ref().as_os_str().to_string_lossy();

        ocl_fs.cache.insert(path_b.into(), k.clone());

        Ok(OclFile {
            global_array_key: k,
        })
    }

    pub fn global_array_key(&self) -> &String {
        &self.global_array_key
    }

    fn read_to_vec(&self) -> io::Result<Vec<u8>> {
        let ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
        let v = ocl_fs.ocl_block.dequeue_buffer(&self.global_array_key)?;
        Ok(v)
    }
}

impl Write for OclFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
        ocl_fs
            .ocl_block
            .enqueue_buffer(buf, &self.global_array_key)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for OclFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let ocl_fs = GLOBAL_OCL_FS.lock().unwrap();
        let v = ocl_fs.ocl_block.dequeue_buffer(&self.global_array_key)?;

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
