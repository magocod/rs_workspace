use std::ops::Deref;
use std::ops::DerefMut;

struct MyBox<T> {
    v: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox { v: x }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

fn display(s: &str) {
    println!("{s}");
}

fn display_mut(s: &mut String) {
    s.push_str("world");
    println!("{s}");
}

fn main() {
    let s = MyBox::new(String::from("hello world"));
    display(&s);

    let mut s = MyBox::new(String::from("hello, "));
    display_mut(&mut s)
}
