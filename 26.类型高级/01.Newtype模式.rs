use std::fmt::{write, Display};

fn main() {
    // Newtype模式
    // rust孤儿规则：只有trait或者类型是本地crate时才允许实现trait
    // 使用Newtyope模式绕过限制
    // 1、使用tuple struct对外部模块进行包装
    // 2、没有性能损失

    struct Wrapper(Vec<String>);
    impl Display for Wrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}]", self.0.join(","))
        }
    }
    let w1 = Wrapper(vec![String::from("2333")]);
    println!("w1 is {w1}")
    // 想为Vec<T>类型实现Display，会违背孤儿规则
    // 现在使用Wrapper这个tuple struct绕过
    // 如果还想使用Vec上更多方法，考虑实现Deref这个Trait，通过解引用访问

    // 还有好处
    // 1、提供更多的类型
    // 2、提供抽象
    // 3、避免混淆(通过对同一基础类型的包装)struct P(u32)，struct M(u32)是两种类型
    // 4、隐藏内部细节（无法直接访问Vec<String>的方法）
}
