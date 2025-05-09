fn main() {
    // 关联类型
    // 1、关联类型将一个‘类型占位符’绑定到trait中
    // 2、允许实现了trait的方法使用这些占位类型
    // 3、实现trait时，由实现者通过具体类型
    trait Foo {
        type Output;
        fn run(&self) -> Self::Output;
    }
    struct Bar {}
    impl Foo for Bar {
        type Output = String;
        fn run(&self) -> Self::Output {
            String::from("指定并实现关联类型")
        }
    }
    let b1 = Bar {};
    b1.run();
}
