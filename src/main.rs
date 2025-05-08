use std::{
    pin::Pin,
    task::{Context, Poll},
};

fn main() {
    // 异步trait Future
    pub trait MyFuture {
        type Output;
        // poll方法会改变future的内部状态所以self是mut的
        // 并且还有通过Pin固定在内存中
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
        // 每次调用.await就相当于调用poll
        // 异步运行时会检查返回值是否Ready，如果没有就继续poll
    }
    enum Poll<T> {
        Ready(T),
        Pending,
    }
}
