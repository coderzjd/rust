use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

fn main() {
    //  Future
    pub trait MyFuture {
        type Output;
        // poll方法会改变future的内部状态所以self是mut的
        // 并且还有通过Pin固定在内存中,因为future内部的Context会有指向自身的引用如果改变地址就是导致野指针
        // 调用.await的时候就会相当于自动实现Pin
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
        // 每次调用.await就相当于调用poll
        // 异步运行时会检查返回值是否Ready，如果没有就继续poll
    }
    enum Poll<T> {
        Ready(T),
        Pending,
    }
}
fn run_pin() {
    // Pin
    // Pin 确保指针指向的数据在内存中位置固定（不可移动）的场景
    // 1、pin是对(类)指针类型(比如：&、&mut、Box、Rc)的包装器
    //      1.1、pin适用于实现了Deref和DerefMut的类型，这实际上等于只适用于指针
    // 2、Pin本身不是指针、也没有像Rc和Arc一样具有引用计数行为
    // 3、它只是编译器用来强制约束指针的工具
}
fn run_join() {
    // Unpin 和 !Unpin
    trpl::run(async {
        let f1 = async { 1 };
        let f2 = async { 2 };
        // trpl::join_all(iter)
        // join_all函数的签名要求集合中的项类型都实现了Future
        // 当Box<T>的T是一个实现了Unpin trait 的future ，Box<T>才实现了Future
        let f_f: Vec<Pin<Box<dyn Future<Output = i32>>>> = vec![Box::pin(f1), Box::pin(f2)];
        let info2 = trpl::join_all(f_f).await;
        // Unpin就是告诉编译器：这个类型的值是可以安全移动的，不用管
        // !Unpin 当一个指向类型的指针被包裹在Pin<T>中，这个T类型必须保证不能移动，才能确保内存安全
        // 一个类型T，组成Pin<T>类型才有Unpin和!Unpin关系
        println!("info is {info2:?}");
    });
}
fn run_stream() {
    // Stream 把iterator和Future整合到一起的类型
    // 1、stream可以像future一样一直Poll
    // 2、可以像iterator一样不断next产生新的项目
    // 3、Stream可能已经结束，返回结果使用Option包裹
    trait Stream {
        // 表示项目产生的类型
        type Item;
        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
    }
    //标准库还没有： StreamExt 拓展类型实现了这些方法
}
