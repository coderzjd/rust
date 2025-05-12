fn main() {
    // Supertrait
    // 如果一个trait依赖于另一个trait的功能,使用Supertrait限定

    trait Foo {
        fn run(&self) {
            println!("foo runing")
        }
    }
    // Bar这个trait的实现调用Foo这个trait的方法
    // 限制了实现Bar这个trait，必须也要实现Foo这个trait
    trait Bar: Foo {
        fn fly(&self) {
            self.run()
        }
    }

    struct Gkd {}
    impl Foo for Gkd {}

    impl Bar for Gkd {}
}
