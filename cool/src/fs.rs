use crate::{GlobalArrayAssigned, OclBlock, DEFAULT_GLOBAL_ARRAY_COUNT, DEFAULT_VECTOR_SIZE};
use std::sync::Mutex;
use lazy_static::lazy_static;
use napi::bindgen_prelude::Buffer;
//
// static GLOBAL_OCL_BLOCK: Mutex<OclBlock> = Mutex::new(OclBlock::new(
//   DEFAULT_VECTOR_SIZE,
//   DEFAULT_GLOBAL_ARRAY_COUNT,
// ));

lazy_static! {
    static ref GLOBAL_OCL_BLOCK: Mutex<OclBlock> = Mutex::new(OclBlock::new(
      DEFAULT_VECTOR_SIZE,
      DEFAULT_GLOBAL_ARRAY_COUNT,
    ));
}

#[derive(Debug)]
pub struct Cache {
  pub path: String,
  pub global_array: GlobalArrayAssigned,
}

#[napi]
pub fn write_file_sync(file: String, data: Buffer) -> napi::Result<()> {
    let ocl_block = GLOBAL_OCL_BLOCK.lock().unwrap();


    Ok(())
}

