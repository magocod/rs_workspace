use std::error::Error;

pub struct Builder {}

impl Builder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> Result<Runtime, Box<dyn Error>> {
        Ok(Runtime {})
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Runtime {}

impl Runtime {
    pub fn start<T>(&self, f: T)
    where
        T: FnOnce(),
    {
        println!("runtime:start");
        f()
    }
}
