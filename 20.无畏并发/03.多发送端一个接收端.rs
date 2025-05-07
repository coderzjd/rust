use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // mpsc是多个生产者一个消费者模型
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        for num in 1..=5 {
            tx.send(num).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        for num in [9; 3] {
            tx1.send(num).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for msg in rx {
        println!("msg is {msg}")
    }
}
