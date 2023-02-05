use rs_proc_macro::show_streams;

// Example: Basic function
#[show_streams]
fn invoke1() {}

// Example: Attribute with input
#[show_streams(bar)]
fn invoke2() {}

// Example: Multiple tokens in the input
#[show_streams(multiple => tokens)]
fn invoke3() {}

// Example:
#[show_streams { delimiters }]
fn invoke4() {}

fn main() {
    println!("example proc_macro_attribute");

    println!("invoke1");
    invoke1();
    println!("invoke2");
    invoke2();
    println!("invoke3");
    invoke3();
    println!("invoke4");
    invoke4();
}
