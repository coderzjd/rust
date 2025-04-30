use std::thread;

fn main() {
    // rust编译器会为闭包实现三种fn trait
    // 1、FnOnce ->  2、FnMut ->  3、Fn
    // 三种trait 存在继承关系

    // 1、FnOnce
    // 只能被调用一次的闭包
    // 所有闭包至少都实现了FnOnce
    // 如果一个闭包将捕获的值移出其主体【通过返回值或者传递给其他函数】
    // 那么它只能实现FnOnce,因为一旦值被移出，闭包不能被再次调用

    // 例如 Option的unwrap_or_else方法声明
    // 闭包的类型为： FnOnce() -> T
    // impl <T>  Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F)
    //     where
    //         F: FnOnce() -> T,
    //     {
    //         match self {
    //             Some(x) => x,
    //             None => f(),
    //         }
    //     }
    // }

    // 2、FnMut
    // 适用于不会移出值、但可能会修改捕获值的闭包，可以被多次调用
    // FnMut 继承于 FnOnce ，实现FnMut 就实现了FnOnce

    // 3、Fn
    // Fn 继承于 FnMut ，实现Fn 就实现了FnMut 实现了FnOnce
    // 适用于不移出值、也不修改捕获的值的闭包
    // 也适用于完全不捕获环境中值的闭包
    // 这类闭包可以被多次调用且不会修改环境
    // 特别适用于需要并发调用闭包的场景

    let s = None;
    let foo = || 1;
    let info = s.unwrap_or_else(foo);
    println!("{}", info);
    let bar = foo();
    println!("{bar}");
}
