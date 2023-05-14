use opencl3::types::cl_int;
// use cl3::types::cl_int;

const LIST_SIZE: u8 = u8::MAX;

pub type ClArray = [cl_int; LIST_SIZE as usize];

pub fn to_array(s: &[u8]) -> ClArray {
    let mut temp: ClArray = [0; LIST_SIZE as usize];

    for (i, v) in s.iter().enumerate() {
        // println!("{i}");
        temp[i] = *v as cl_int;
    }

    temp
}

pub fn hello() -> String {
    "hello".to_string()
}

pub mod error;

pub mod ocl_v1;
pub mod ocl_v2;
pub mod ocl_v3;
pub mod ocl_v4;
pub mod ocl_v5;
pub mod ocl_v6;

pub mod ocl_fs;
pub mod ocl_fs_2;

pub mod process;
pub mod server;
pub mod utils;

// re-export
pub mod open_cl3 {
    pub use opencl3::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = hello();
        assert_eq!(result, "hello");
    }
}
