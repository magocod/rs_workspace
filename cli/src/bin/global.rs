use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_ID: usize = usize::MAX / 2;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

#[derive(Debug)]
struct Factory {
    factory_id: usize,
}

impl Factory {
    pub fn new() -> Self {
        Self {
            factory_id: generate_id(),
        }
    }

    pub fn get_id(&self) -> usize {
        self.factory_id
    }
}

impl Default for Factory {
    fn default() -> Self {
        Self::new()
    }
}

fn generate_id() -> usize {
    let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if current_val > MAX_ID {
        panic!("Factory ids overflowed");
    }
    let next_id = GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    if next_id > MAX_ID {
        panic!("Factory ids overflowed");
    }
    next_id
}

fn main() {
    let v = generate_id();
    println!("id {v}");

    let f = Factory::new();
    println!("factory {f:?}");
    println!("id {}", f.get_id());

    // First access to `HASHMAP` initializes it
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

    // Any further access to `HASHMAP` just returns the computed value
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());
}
