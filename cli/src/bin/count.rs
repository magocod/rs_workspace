use shared::counter::Counter;

fn main() {
    println!("counter iterator");
    let mut counter = Counter::new();

    // A
    println!("{counter:?}");
    println!("next {:?}", counter.next());
    println!("next {:?}", counter.next());
    println!("next {:?}", counter.next());
    println!("next {:?}", counter.next());
    println!("next {:?}", counter.next());
    println!("next {:?}", counter.next());
    println!("{counter:?}");

    // B
    let counter = Counter::new();
    for v in counter {
        println!("next {v}");
    }
}
