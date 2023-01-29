pub use hello_macro_derive::*;

pub trait HelloMacro {
    fn hello_macro();
    fn hello(&self);
}
