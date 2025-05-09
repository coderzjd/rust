fn main() {
    // 完全限定语法
    // 1、Rust允许多个Trait拥有相同方法名称
    // 2、也可以在同一个类型上实现多个带有相同方法名称的Trait
    // 3、类型本身也可以定义相同名称的方法
    struct Foo {}
    struct Bar {}
    impl Foo {
        fn run() {}
    }
    impl Bar {
        fn run() {}
    }
}
