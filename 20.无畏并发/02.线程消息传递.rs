use std::{sync::mpsc, thread};

fn main() {
    // 消息传递
    // 线程或者actors通过发送包含数据的消息通信
    // Go语言: 不要通过共享内存来通信、而是通过通信来共享内存
    // Rust标准库提供了通道（channel）实现

    // channel:
    // 1、通道是一种设计概念、用于不同线程之间发送数据
    // 2、核心部分： 发送端（transmitter）和接收端（receiver）
    // 3、当通道的任意一端被丢失，表示通道关闭
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let s1 = String::from("998");
        // send方法接受一个参数，返回值为Result类型
        // 如果接收端被丢弃，send方法返回一个错误
        tx.send(s1).unwrap();
        // 传递消息存所有权转移
        // println!("s1 is {s1}");
    });
    // 接收端：
    // recv方法
    // 1、会阻塞当前线程，直到收到一个值
    // 2、一旦有值被发送过来就会返回一个包含改值的Result
    // 3、当所有发送端都关闭、会返回一个错误
    let msg = rx.recv().unwrap();
    println!("接受的 msg 是 {msg}");

    // try_recv方法
    // 1、不会阻塞，立即返回一个Result
    // 2、如果当前有消息可用，则返回Ok包含消息
    // 3、如果当前没有消息可用，返回Err
    // let msg2 = rx.try_recv().unwrap();
    // println!("接受的 msg2 是 {msg2}")
}
