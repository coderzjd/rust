use std::thread;

fn main() {
    // 闭包捕获引用或移动所有权
    // 1、不可变借用
    // s1为不可变引用
    let s1 = String::from("2333");
    println!("s1 is {s1}");
    let foo = || println!("foo s1 is {s1}");
    println!("s1 is {s1}");
    foo();
    println!("s1 is {s1}");

    // 2、可变借用 [声明和使用可变借用闭包：相当于调用参数为可变引用的函数]
    let mut s2 = String::from("2333");
    println!("s2 is {s2}");
    // 此处foo为可变引用闭包
    let mut foo = || s2.push_str("_998");
    // 在foo使用前无法借用为不可变引用
    // println!("s2 is {s2}");
    foo();
    println!("s2 is {s2}");

    // 3、取得所有权
    let s3 = String::from("998");
    println!("s3 is {}", s3);
    // 开辟新线程获得主线程的变量所有权使用move
    // 新线程和主线程的结束时间不一致
    thread::spawn(move || println!("捕获s3 {}", s3))
        .join()
        .unwrap();
    // 所有权转移后无法使用变量
    // println!("s3 is {}", s3);
}
