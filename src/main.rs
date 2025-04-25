#[derive(Debug)]
struct Point(i32, String);
// 使用impl为结构体实现方法
impl Point {
    fn incr_x(&mut self) {
        self.0 += 1
    }
    fn push_y(&mut self) {
        self.1.push_str("998");
    }
}
fn main() {
    let mut p = Point(1, String::from("233"));
    p.incr_x();
    println!("{}", p.0);
    p.push_y();
    println!("{}", p.1)
}
