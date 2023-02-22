// #![allow(non_snake_case)]
#![allow(dead_code)]

pub mod gpu_performance;

const LIB_PATH: &str = "../lib/Dll1/x64/Debug/Dll1.dll";

extern "C" {
    fn hello_c();
    // fn hello_cpp();
}

pub fn call_c() {
    unsafe {
        hello_c();
    }
}

// pub fn call_cpp() {
//     unsafe {
//         hello_cpp();
//     }
// }

// #[link(name = "Dll1", kind = "static")]
// extern {
//     fn fooBar(arg: i64) -> i64;
// }
//
// fn call_dll(v: i64) {
//     unsafe {
//         let r = fooBar(v);
//         println!("{}", r);
//     }
// }

pub type FooBarFn = unsafe extern "C" fn(v: i64) -> i64;

#[cfg(target_os = "windows")]
fn call_dynamic() -> Result<i64, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("Dll1.dll")?;
        let func: libloading::Symbol<FooBarFn> = lib.get(b"fooBar")?;
        Ok(func(1))
    }
}

fn call_dynamic_with_path() -> Result<i64, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new(LIB_PATH)?;
        let func: libloading::Symbol<FooBarFn> = lib.get(b"fooBar")?;
        Ok(func(1))
    }
}

pub struct DllWrapper {
    lib: libloading::Library,
}

impl DllWrapper {
    pub fn new(path: &str) -> Self {
        unsafe {
            let lib = libloading::Library::new(path).unwrap();
            Self { lib }
        }
    }

    pub fn foo_bar(&self, v: i64) -> Result<i64, Box<dyn std::error::Error>> {
        unsafe {
            let func: libloading::Symbol<FooBarFn> = self.lib.get(b"fooBar")?;
            Ok(func(v))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_fn_from_c() {
        call_c();
        // call_cpp();
    }

    // #[test]
    // fn rust_call_dll() {
    //     call_dll(1);
    // }

    #[test]
    #[cfg(target_os = "windows")]
    fn rust_call_dll() {
        let f = call_dynamic().unwrap();
        println!("{f}");
        assert_eq!(f, 6);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn rust_call_dll_with_path() {
        let f = call_dynamic_with_path().unwrap();
        println!("{f}");
        assert_eq!(f, 6);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn rust_call_dll_wrapper() {
        let w = DllWrapper::new(LIB_PATH);
        let r = w.foo_bar(0).unwrap();

        assert_eq!(r, 5);
    }
}
