fn main() {
    // 得到debug宏
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let c1 = Color(255, 0, 255);
    let p1 = Point(1, 2, 3);
    println!("c1 取值{}", c1.0);
    println!("p1 取值{}", p1.0)
}
