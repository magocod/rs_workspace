fn main() {
    println!("simple builder");

    rs_core::runtime::Builder::new()
        .build()
        .expect("Failed building the Runtime")
        .start(|| println!("call runtime.start closure"));
}
