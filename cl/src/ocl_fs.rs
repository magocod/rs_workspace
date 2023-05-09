use std::collections::HashMap;
use crate::ocl_v5::{GlobalArrayMap, OpenClBlock};

#[derive(Debug)]
pub struct OpenClFs {
    inner: OpenClBlock,
    cache: HashMap<String, GlobalArrayMap>,
}

#[derive(Debug)]
pub struct FileCache {
    pub index: u32,
    pub size: u64,
}
