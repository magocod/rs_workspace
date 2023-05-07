#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::{Buffer};
use cl::ocl_v4::OpenClBlock;
use cl::open_cl3::types::cl_int;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi(js_name = "OclBlock")]
pub struct OclBlock {
  inner: OpenClBlock,
}

#[napi]
impl OclBlock {
  #[napi(constructor)]
  pub fn new() -> Self {
    OclBlock { inner: OpenClBlock::new().expect("OpenClBlock::new() failed") }
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
    self.inner.enqueue_buffer(
      &k,
      js_buffer.as_ref(),
      index
    )?;
    Ok(index as u32)
  }

  #[napi]
  pub fn dequeue_buffer(&mut self, global_array_index: u32) -> napi::Result<Buffer> {
    let k = self.inner.create_vector_extract_kernel();
    let v = self.inner.dequeue_buffer(
      &k,
      global_array_index as cl_int,
    )?;
    Ok(Buffer::from(v))
  }
}
