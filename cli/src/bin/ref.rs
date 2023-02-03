use std::cell::RefCell;
use std::rc::Rc;

pub trait Messenger {
    fn send(&self, msg: String);
}

#[derive(Debug)]
pub struct MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg)
    }
}

fn main() {
    let mq = MsgQueue {
        msg_cache: RefCell::new(Vec::new()),
    };
    println!("MsgQueue {mq:?}");
    mq.send("hello".to_string());
    mq.send("world".to_string());
    println!("MsgQueue {mq:?}");

    let s = Rc::new(RefCell::new("a, b".to_string()));

    let s1 = s.clone();
    let s2 = s.clone();
    // let mut s2 = s.borrow_mut();
    s2.borrow_mut().push_str(", c");

    println!("{s:?}\n{s1:?}\n{s2:?}");
}
