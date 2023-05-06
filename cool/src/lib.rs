#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use cl::ocl_v4::OpenClBlock;

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
  pub fn query(&self, query: String) -> napi::Result<String> {
    // self.inner.enqueue_buffer()
  }
}
