use std::sync::Mutex;

pub static GLOBAL_VEC: Mutex<Vec<i32>> = Mutex::new(Vec::new());

#[napi]
pub fn add_global_vec(a: i32) -> napi::Result<()> {
  GLOBAL_VEC.lock().unwrap().push(a);
  Ok(())
}

#[napi]
pub fn show_global_vec() -> napi::Result<()> {
  println!("{GLOBAL_VEC:?}");
  Ok(())
}

#[napi]
pub fn get_global_vec() -> napi::Result<Vec<i32>> {
  Ok(GLOBAL_VEC.lock().unwrap().clone())
}
