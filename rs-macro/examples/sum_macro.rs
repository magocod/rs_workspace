use rs_macro::add;

fn main() {
    println!("Hello from an example macro!");
    let d = add!(1, 2);
    println!("r - {d}");
}
