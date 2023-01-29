use heapsize::HeapSize;
use hello_macro::HelloMacro;
use shared::sample::Sample;

#[derive(Debug)]
struct Example {}

impl Sample for Example {
    fn sample(&self) {
        println!("call sample trait")
    }
}

#[derive(Debug, HelloMacro)]
struct ExampleDerive {}

#[derive(HeapSize)]
struct Demo<'a, T: ?Sized> {
    a: Box<T>,
    b: u8,
    c: &'a str,
    d: String,
}

fn main() {
    println!("bin Derive");

    let d = Example {};

    println!("struct - {:?}", d);
    d.sample();

    let m = ExampleDerive {};

    println!("struct - derive HelloMacro, - {:?}", m);
    ExampleDerive::hello_macro();
    m.hello();

    let demo = Demo {
        a: b"bytestring".to_vec().into_boxed_slice(),
        b: 255,
        c: "&'static str",
        d: "String".to_owned(),
    };

    // 10 + 0 + 0 + 6 = 16
    println!(
        "heap size = {} + {} + {} + {} = {}",
        demo.a.heap_size_of_children(),
        demo.b.heap_size_of_children(),
        demo.c.heap_size_of_children(),
        demo.d.heap_size_of_children(),
        demo.heap_size_of_children()
    );
}
