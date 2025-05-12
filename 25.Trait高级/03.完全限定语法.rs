fn main() {
    // 完全限定语法
    // 1、Rust允许多个Trait拥有相同方法名称
    // 2、也可以在同一个类型上实现多个带有相同方法名称的Trait
    // 3、类型本身也可以定义相同名称的方法

    trait Foo {
        fn run(&self) {
            println!("trait Foo 的 run 方法")
        }
        fn fly() {
            println!("飞起来了1111")
        }
    }
    trait Bar {
        fn run(&self) {
            println!("trait Bar 的 run 方法")
        }
    }
    struct Gkd {}
    impl Gkd {
        fn run(&self) {
            println!("Gkd 自己实现的 run 方法")
        }
        fn fly() {
            println!("飞起来了2222")
        }
    }
    impl Foo for Gkd {
        fn fly() {
            println!("飞起来了3333")
        }
    }
    impl Bar for Gkd {}
    let g1 = Gkd {};
    g1.run();
    // 通过关联函数调用trait上同名方法
    Foo::run(&g1);
    Bar::run(&g1);

    // 使用as关键字调用trait同名方法
    Gkd::fly();
    <Gkd as Foo>::fly()
}
