use std::fmt::Display;

fn main() {
    // 例子： 内置的i32类型实现了to_string方法
    let info = 3.to_string();
}
// blanket实现
// 为实现了某个trait的类型有条件的实现另一个trait
// 为实现了Display约束的类型实现ToString这个trait
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         format!("{}", self)
//     }
// }
