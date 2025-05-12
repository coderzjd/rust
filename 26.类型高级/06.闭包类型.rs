fn main() {
    // 闭包的类型
    // 1、闭包没有类型，不能直接返回
    // 2、每个闭包都要自己独特的类型
    // 3、使用impl Trait实现
    fn return_close() -> impl Fn(i32) -> i32 {
        |x: i32| -> i32 { x + 1 }
    }
    // 使用trait Object
    type AddN = Box<dyn Fn(i32) -> i32>;

    fn addn(n: i32) -> AddN {
        let aa = move |x: i32| -> i32 { n + x };
        Box::new(aa)
    }
}
