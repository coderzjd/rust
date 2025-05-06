fn main() {
    // 编译器会根据上下文：为每个参数和返回值推断出具体的类型，类型不可改变
    let foo = |x| x;
    // 这里推断出foo闭包的参数x为String类型
    let s = foo(String::from("998"));
    // 这里调用x为i32类型，类型不匹配报错
    // let n = foo(1);

    let bar = |_| (); //厕所闭包，不知道什么意思?
    let info = String::from("23333");
    bar(info);
}
