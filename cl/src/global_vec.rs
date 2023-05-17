use std::sync::Mutex;

pub static GLOBAL_VEC: Mutex<Vec<i32>> = Mutex::new(Vec::new());

pub fn add_global_vec(v: i32, th: i32) -> () {
    println!("Thread #{th} exec add_global_vec");
    GLOBAL_VEC.lock().unwrap().push(v);
    println!("Thread #{th} lock add_global_vec");
    ()
}

pub fn get_global_vec(i: i32, th: i32) -> i32 {
    println!("Thread #{th} exec add_global_vec");
    let v = GLOBAL_VEC.lock().unwrap();
    println!("Thread #{th} lock get_global_vec");
    let r = v[i as usize];
    r
}

pub fn print_global_vec(th: i32) -> () {
    println!("Thread #{th} exec print_global_vec");
    let v = GLOBAL_VEC.lock().unwrap();
    println!("Thread #{th} lock print_global_vec");
    println!("{v:?}");
}

pub fn copy_global_vec(th: i32) -> Vec<i32> {
    println!("Thread #{th} exec copy_global_vec");
    let v = GLOBAL_VEC.lock().unwrap();
    println!("Thread #{th} lock copy_global_vec");
    v.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn sync() {
        add_global_vec(1, 1);
        let v = get_global_vec(0, 1);
        let copy = copy_global_vec(1);
        add_global_vec(v + 1, 1);
        println!("{:?}", copy);
        print_global_vec(1);
    }

    #[test]
    fn multi_thread() {
        let total_threads = 10;
        add_global_vec(1, 1);

        let threads: Vec<_> = (0..total_threads)
            .map(|i| {
                thread::spawn(move || {
                    println!("Thread #{i} started!");
                    let i = rand::thread_rng().gen_range(0..4);

                    if i == 0 {
                        add_global_vec(i, 1);
                    } else if i == 1 {
                        let v = get_global_vec(0, 1);
                        println!("v {:?}", v);
                    } else if i == 2 {
                        print_global_vec(1)
                    } else {
                        let copy = copy_global_vec(1);
                        println!("{:?}", copy);
                    }

                    thread::sleep(Duration::from_millis(500));
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
}
