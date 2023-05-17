use cl::global_vec::{add_global_vec, copy_global_vec, get_global_vec, print_global_vec};
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    let total_threads = 10;
    add_global_vec(1, -1);

    let threads: Vec<_> = (0..total_threads)
        .map(|i| {
            thread::spawn(move || {
                println!("Thread #{i} started!");
                let r = rand::thread_rng().gen_range(0..4);
                thread::sleep(Duration::from_millis(1000));

                if r == 0 {
                    add_global_vec(i, i);
                } else if r == 1 {
                    let v = get_global_vec(0, i);
                    println!("v {:?}", v);
                } else if r == 2 {
                    print_global_vec(i)
                } else {
                    let copy = copy_global_vec(i);
                    println!("{:?}", copy);
                }

                thread::sleep(Duration::from_millis(1000));
                println!("Thread #{i} finished!");
            })
        })
        .collect();

    for handle in threads {
        handle.join().unwrap();
    }

    thread::sleep(Duration::from_millis(1000));
    println!("Thread #main finished!");
}
