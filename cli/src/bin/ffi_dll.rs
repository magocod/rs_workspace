use ffi::DllWrapper;

const LIB_PATH: &str = "../../lib/Dll1/x64/Debug/Dll1.dll";

fn main() {
    let w = DllWrapper::new(LIB_PATH);
    let r = w.foo_bar(5).unwrap();

    println!("{r}");
}
