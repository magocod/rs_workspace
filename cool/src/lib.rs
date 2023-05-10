#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use cl::ocl_v5::{OpenClBlock, LIST_SIZE, TOTAL_GLOBAL_ARRAY};
use napi::bindgen_prelude::{BigInt, Buffer};

pub mod fs;
pub mod global;

#[napi]
pub const DEFAULT_VECTOR_SIZE: u32 = LIST_SIZE as u32;
#[napi]
pub const DEFAULT_GLOBAL_ARRAY_COUNT: u32 = TOTAL_GLOBAL_ARRAY as u32;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[derive(Debug)]
#[napi(js_name = "OclBlock")]
pub struct OclBlock {
  inner: OpenClBlock,
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct GlobalArrayAssigned {
  pub index: u32,
  pub size: BigInt, // u64
}

#[napi]
impl OclBlock {
  #[napi(constructor)]
  pub fn new(vector_size: u32, global_array_count: u32) -> Self {
    OclBlock {
      inner: OpenClBlock::new(vector_size as usize, global_array_count as usize)
        .expect("OpenClBlock::new() failed"),
    }
  }

  pub fn opencl_block(&self) -> &OpenClBlock {
    &self.inner
  }

  #[napi]
  pub fn initialize(&self) -> napi::Result<()> {
    let _ = self.inner.create_vector_add_kernel();
    let _ = self.inner.create_vector_extract_kernel();
    Ok(())
  }

  #[napi]
  pub fn enqueue_buffer(
    &mut self,
    js_buffer: Buffer,
    global_array_index: Option<u32>,
  ) -> napi::Result<u32> {
    let k = self.inner.create_vector_add_kernel();

    let index = match global_array_index {
      None => self.inner.get_global_array_index()?,
      Some(v) => v,
    };
    self.inner.enqueue_buffer(&k, js_buffer.as_ref(), index)?;
    Ok(index as u32)
  }

  #[napi]
  pub fn dequeue_buffer(&self, global_array_index: u32) -> napi::Result<Buffer> {
    let k = self.inner.create_vector_extract_kernel();
    let v = self.inner.dequeue_buffer(&k, global_array_index)?;
    Ok(Buffer::from(v))
  }

  // #[napi]
  // pub fn get_global_arrays(&self) -> napi::Result<Vec<GlobalArrayAssigned>> {
  //   let map = self.inner.get_global_arrays();
  //   let mut vec = Vec::with_capacity(map.len());
  //
  //   for (index, size) in map {
  //     vec.push(GlobalArrayAssigned {
  //       index: *index,
  //       size: BigInt::from(*size),
  //     });
  //   }
  //
  //   Ok(vec)
  // }

  #[napi]
  pub fn get_global_arrays(&self) -> napi::Result<Vec<GlobalArrayAssigned>> {
    let map = self.inner.get_global_arrays();
    let mut vec = Vec::with_capacity(map.len());

    for (index, size) in map {
      vec.push(GlobalArrayAssigned {
        index: *index,
        size: BigInt::from(*size),
      });
    }

    Ok(vec)
  }
}

// TODO TESTS
