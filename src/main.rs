use std::ops::Deref;

fn main() {
    // Box<T>
    // 1、允许将数据存储在heap上，在stack上指针指向heap数据的指针
    // 适用场景
    // 1、在需要知道确切大小的上下文中、却使用一个在编译时无法确定大小的类型
    // 2、有大量数据，想转移所有权，但需确保在转移时数据不会被复制
    // 3、当你想拥有一个值、且你只关心它是否实现了某个Trait，而不是具体的类型

    let b1 = Box::new(1);
    let b2 = b1;
    // 存储在heap上已经丢失所有权
    // println!("b1 is {b1}");
    println!("b2 is {b2}");

    // 递归类型“List”的大小是无限的,无法确定大小
    // enum List {
    //     Comns(i32, List),
    //     Nil,
    // }
    // Box<T> 存储的指针、此时大小固定
    enum List {
        Comns(i32, Box<List>),
        Nil,
    }
    use List::{Comns, Nil};
    let l1 = Comns(1, Box::new(Nil));

    // 自定义MyBox<T>类型
    let m1 = MyBox::new(1);
    let m2 = *m1;
    println!("m2 is {m2}")
}
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}
// MyBox实现了Deref这个Trait，可以使用解引用操作
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
