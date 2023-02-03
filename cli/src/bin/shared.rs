use shared::Dummy;

fn main() {
    println!("bin shared");

    let d = Dummy {};

    println!("struct - {d:?}");
}
