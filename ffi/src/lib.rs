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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_fn_from_c() {
        call_c();
        // call_cpp();
    }
}
