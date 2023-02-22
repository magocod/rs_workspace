#![allow(non_snake_case)]

const GPU_PERFORMANCE_LIB_PATH: &str = "GPUPerfAPIDX12-x64.dll";

#[derive(Debug)]
pub struct GpuPerformance {
    lib: libloading::Library,
}

#[derive(Debug)]
pub struct GPAApiManager {
    Instance: fn() -> GPAApiManager,
    LoadApi: fn() -> i32
}

impl GpuPerformance {
    pub fn new(path: &str) -> Self {
        unsafe {
            let lib = libloading::Library::new(path).unwrap();
            Self { lib }
        }
    }

    pub fn gpa_api_manager(&self) -> Result<i64, Box<dyn std::error::Error>> {
        unsafe {
            let manager: libloading::Symbol<GPAApiManager> = self.lib.get(b"GPAApiManager")?;
            let m = manager.Instance;
            let i = m().LoadApi;
            let v = i();
            println!("i {v:?}");

            Ok(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn call_gpu_performance_instance() {
        let gpu_performance = GpuPerformance::new(GPU_PERFORMANCE_LIB_PATH);
        let _ = gpu_performance.gpa_api_manager();
    }
}