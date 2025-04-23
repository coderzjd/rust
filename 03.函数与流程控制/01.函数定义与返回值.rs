fn main() {
    // fn定义函数
    // 函数名称使用小写+单词之间使用下划线(_)连接
    // 函数使用： ->  表示返回值类型
    fn print_function(info: i32, other: i32) -> i32 {
        println!("other is {}", info);
        println!("other is {}", other);
        // 主动return
        // return  1;
        //也可以省略return; 函数的返回值默认为最后一行表达式的结果
        99
    }
    let info = print_function(1, 2);
    println!("info is {}", info);

    fn foo(x: (i32, i32)) {
        println!("x is {:?}", x)
    }
    foo((1, 2));
    fn bar((a, b): (i32, i32)) {
        println!("a is {},b is {}", a, b)
    }
    bar((1, 2));

    fn gkd() {}
    {
        fn gkd() {}
    }
    // 函数名称在一个作用域内不能重复
    // fn gkd() {}
    // let 关键字可以shadowing函数名称
    let gkd = 1;
    println!(" gkd is {gkd}");
}
