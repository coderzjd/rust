fn main() {
    // 完全限定语法
    // 1、Rust允许多个Trait拥有相同方法名称
    // 2、也可以在同一个类型上实现多个带有相同方法名称的Trait
    // 3、类型本身也可以定义相同名称的方法

    trait Foo {
        fn run(&self) {
            println!("trait Foo 的 run 方法")
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
    }
    impl Foo for Gkd {}
    impl Bar for Gkd {}
    let g1 = Gkd {};
    g1.run();
    Foo::run(&g1);
    Bar::run(&g1);
}
