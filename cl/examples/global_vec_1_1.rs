use rand::Rng;
use std::thread;
use std::time::Duration;

pub fn add_global_vec(v: i32, th: i32) -> () {
    println!("Thread #{th} exec add_global_vec");
    // pass
    println!("Thread #{th} lock add_global_vec");
    ()
}

pub fn get_global_vec(i: i32, th: i32) -> i32 {
    println!("Thread #{th} exec add_global_vec");
    // pass
    println!("Thread #{th} lock get_global_vec");
    1
}

pub fn print_global_vec(th: i32) -> () {
    println!("Thread #{th} exec print_global_vec");
    // pass
    println!("Thread #{th} lock print_global_vec");
}

pub fn copy_global_vec(th: i32) -> Vec<i32> {
    println!("Thread #{th} exec copy_global_vec");
    // pass
    println!("Thread #{th} lock copy_global_vec");
    vec![]
}

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
