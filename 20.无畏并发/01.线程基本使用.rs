use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

fn main() {
    // RUST的无畏并发
    // 1、高效安全的并发编程
    // 2、利用所有权和类型系统在编译时发现错误
    // 3、开发阶段发现错误
    // 4、为不同的并发模型提供多种工具

    // 多线程问题（错误难以复现）
    // 1、竞态条件：线程以不一致的顺序访问数据
    // 2、死锁：两个线程相互等待，导致两个线程都无法继续

    // rust标准库线程为1:1,每创建一个线程就会创建一个系统线程

    // 当主线程运行完毕时会关闭子线程，不管子线程是否运行完毕
    // 线程的顺序是不一定的，依赖操作系统进行调度

    // JoinHandle是一个有所有权的值，调用join方法会阻塞当前进行的线程，知道handler代表的线程终止
    let handler = thread::spawn(|| {
        for item in 1..=10 {
            println!("子线程数字: {item}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for item in 1..=5 {
        println!("主线程: {item}");
        thread::sleep(Duration::from_millis(1));
    }
    handler.join().unwrap();

    // 获取线程的返回值
    let t = thread::spawn(|| "线程的返回值");
    let info = t.join().unwrap();
    println!("获取 :{}", info)
}
