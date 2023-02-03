use rs_extra::add_conditional;

fn main() {
    println!("bin A - with optional dependencies");
    let v = add_conditional(1, 1);
    println!("v {v}");
}
