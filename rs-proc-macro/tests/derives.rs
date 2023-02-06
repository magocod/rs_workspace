use rs_proc_macro::{add_field, Counter, Hello, Process};

#[derive(Hello)]
struct Example {}

#[add_field]
#[derive(Debug)]
struct ExampleProp {}

// #[add_field]
// fn field() {
//
// }

#[derive(Process)]
struct Processor {
    value: u8,
}

impl Processor {
    fn step_a(&self) -> u8 {
        self.value
    }
}

#[derive(Counter)]
struct Cache {
    count: i32,
}

#[test]
fn derive_hello() {
    let e = Example {};
    assert_eq!(e.hello(), "Hello, My name is Example!");
}

#[test]
fn macro_add_field() {
    let e = ExampleProp {
        a: "abc".to_string(),
    };
    assert_eq!(e.a, "abc".to_string());
}

#[test]
fn derive_process() {
    let p = Processor { value: 0 };
    let p2 = Processor { value: 3 };
    assert_eq!(p.step_b(), 1);
    assert_eq!(p2.step_b(), 4);
}

#[test]
fn derive_counter() {
    let cd = Cache::default();
    let mut c = Cache::new();
    c.increment();
    let mut c2 = Cache::new();
    c2.decrement();

    assert_eq!(cd.state(), 0);
    assert_eq!(c.state(), 1);
    assert_eq!(c2.state(), -1);
}
