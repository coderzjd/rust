use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    // Arc<T> 是类似于 Rc<T> 的类型，可以在并发情况下安全使用
    // Rc<T>单线程使用
    // Arc<T> 多线程使用
    let count = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _item in 1..=10 {
        let counter = Arc::clone(&count);
        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1
        });
        handles.push(handler);
    }
    for item in handles {
        item.join().unwrap();
    }
    println!("count is {:?}", count.lock().unwrap());
}
