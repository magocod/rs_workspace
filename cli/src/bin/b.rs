use rs_core::cache::get;

fn main() {
    println!("bin B - with feature");
    let v = get();
    println!("v {v}");
}
