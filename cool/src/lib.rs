#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use cl::ocl_v5::OpenClBlock;
use napi::bindgen_prelude::{BigInt, Buffer};

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi(js_name = "OclBlock")]
pub struct OclBlock {
  inner: OpenClBlock,
}

#[napi(object)]
pub struct GlobalArrayAssigned {
  pub index: u32,
  pub size: BigInt, // u64
}

#[napi]
impl OclBlock {
  #[napi(constructor)]
  pub fn new() -> Self {
    OclBlock {
      inner: OpenClBlock::new().expect("OpenClBlock::new() failed"),
    }
  }

  #[napi]
  pub fn initialize(&self) -> napi::Result<()> {
    let _ = self.inner.create_vector_add_kernel();
    let _ = self.inner.create_vector_extract_kernel();
    Ok(())
  }

  #[napi]
  pub fn enqueue_buffer(&mut self, js_buffer: Buffer) -> napi::Result<u32> {
    let k = self.inner.create_vector_add_kernel();
    let index = self.inner.get_global_array_index()?;
    self.inner.enqueue_buffer(&k, js_buffer.as_ref(), index)?;
    Ok(index as u32)
  }

  #[napi]
  pub fn dequeue_buffer(&mut self, global_array_index: u32) -> napi::Result<Buffer> {
    let k = self.inner.create_vector_extract_kernel();
    let v = self
      .inner
      .dequeue_buffer(&k, global_array_index)?;
    Ok(Buffer::from(v))
  }

  #[napi]
  pub fn get_global_arrays(&self) -> napi::Result<Vec<GlobalArrayAssigned>> {
    let map = self.inner.get_global_arrays();
    let mut vec = Vec::with_capacity(map.len());

    for (index , size) in map {
      vec.push(GlobalArrayAssigned { index: *index, size: BigInt::from(*size) });
    }

    Ok(vec)
  }
}
