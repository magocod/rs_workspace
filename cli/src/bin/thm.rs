use std::thread;

fn child_job(s: &mut String) {
    *s = s.to_uppercase();
}

fn main() {
    let strings = vec![
        "hello".to_string(),
        "world".to_string(),
        "testing".to_string(),
        "good enough".to_string(),
    ];

    // start the threads, giving them the strings
    let mut thread_handles = vec![];
    for mut s in strings {
        thread_handles.push(thread::spawn(move || {
            child_job(&mut s);
            s
        }));
    }

    // wait for threads and re-populate `strings`
    let strings = thread_handles.into_iter().map(|h| h.join().unwrap());

    // print result
    for s in strings {
        println!("{s}");
    }
}
