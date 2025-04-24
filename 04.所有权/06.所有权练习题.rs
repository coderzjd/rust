use std::rc::Rc;
fn main() {
    // 此时s1的类型为Rc<String>，当main函数被回收，变量也会被回收
    let s1: Rc<String> = get_string();
    println!("s1 is {}", s1)
}
// fn get_string() -> &String {
//     // 函数允许完毕s2被释放，无法返回引用
//     let s2 = String::from("11");
//     &s2
// }
// 进行所有权转移是可以的
fn get_string() -> Rc<String> {
    // Rc会对产生的String变量进行引用计数，当计数为0时销毁变量
    // Rc::new()当前计数为1
    let s2 = Rc::new(String::from("11"));
    // Rc::clone()方法会让计数变成2
    Rc::clone(&s2)
    // 函数回收后Rc计数变成1
}
