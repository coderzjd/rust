use std::fmt::Display;

fn main() {
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        // 所有的Pair实例都有new方法
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    // 只有实现了{ Display + PartialOrd}类型的Pair实例才有cmp_display方法
    impl<T: Display + PartialOrd> Pair<T> {
        // 因为T的类型约束为 {Display + PartialOrd},所以可以比大小和支持输出
        fn cmp_display(&self) {
            if &self.x >= &self.y {
                println!("x 最大 {}", &self.x)
            } else {
                println!("y 最大 {}", &self.y)
            }
        }
    }
}
