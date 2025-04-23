fn main() {
    // 1.语句与表达式
    //      1、语句是执行一些操作
    //      2、表达式有返回值
    // let 声明变量是不可变的
    let aaa = 1;
    // aaa = 2;

    // mut 关键字声明可变变量
    let mut bbb = 1;
    bbb = 2;
    // shadowing特性
    // 支持定义同名变量
    // 变量的类型和mut都可以不同
    let myname = "111";
    println!("{myname}");
    let mut myname = 222;
    // 2、作用域和js类似有函数作用域和块级作用域 {}
    {
        let myname = '我';
        println!("{myname}")
    }
    println!("{myname}");
    // const 关键字声明常量
    // 不可与使用mut
    // 变量名必须大写
    // 必须指定类型
    const INFO_A: u8 = 10;
    // 只能定义一次不支持shadowing
    // const INFO_A: u8 = 10;
    // 只能使用常量表达式赋值(在运行前就知道值)
    // const INFO_B: i32 = aaa;
    fn foo() {
        // 在不同作用域内不冲突
        const INFO_A: u8 = 10;
    }
}
