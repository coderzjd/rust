fn main() {
    // Unit-Like Structs
    // 可以用于实现trait
    #[derive(Debug)]
    struct AlwaysEqual;

    let aa = AlwaysEqual;
    println!("{:?}", aa);

    // 定义结构体Gkd的时候多些了一个括号，不知道为什么？
    // 好像是写错啦把枚举类型当函数调用了
    #[derive(Debug)]
    struct Gkd();
    let aa = Gkd;
    println!("{:?}", aa());
}
