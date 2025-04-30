fn main() {
    let num = 8;
    // 不能使用fn函数闭包捕获变量
    // fn bar() {
    //     println!("num is {}", num)
    // }
    // 只能使用闭包函数
    let foo = || println!("{num}");
    // 闭包函数调用和普通函数一样
    foo();
    // 声明闭包附带参数类型
    let gkd = |x: i32| -> i32 { x + 1 };
    // 闭包运行体只要有一行可以省略大括号{类似js箭头函数}
    let gkd = |x: i32| x + 1;
    // 闭包不带参数
    let gkd = || println!("num is {num}");
    // 闭包的简单使用
    let p1 = None;
    let info = p1.unwrap_or_else(|| 2);
    println!("info is {}", info)
}
