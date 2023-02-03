use shared::callback::{GProcessor, Processor};

fn simple_callback() {
    println!("hello world!");
}

fn main() {
    let p = Processor {
        callback: simple_callback,
    };
    p.process_events(); // hello world!

    let s = "world!".to_string();
    let callback = || println!("hello {s}");
    let mut p = GProcessor { callback };
    p.process_events();
}
