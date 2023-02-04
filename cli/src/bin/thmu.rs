use std::sync::{Arc, Mutex};
use std::thread;

fn count() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn vec_push() {
    let v = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for i in 0..10 {
        let vc = Arc::clone(&v);
        let handle = thread::spawn(move || {
            let mut v_lock = vc.lock().unwrap();

            v_lock.push(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {:?}", *v.lock().unwrap());
}

fn main() {
    count();
    vec_push();
}
