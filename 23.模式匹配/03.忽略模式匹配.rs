fn main() {
    // 忽略模式匹配
    fn foo(_: i32, y: i32) {
        // 忽略第一个值
        println!("y is {y}")
    }
    match (Some(1), Some(2)) {
        // 忽略多个值
        (Some(_), Some(_)) => {}
        _ => {}
    }
    let tup = (1, 2, 3, 4, 5);
    match tup {
        (x, _, y, _, z) => println!("x is {x}, y is {y}, z is {z}"),
        _ => println!(" match other"),
    }

    // 忽略匹配与所有权
    let s1 = Some(String::from("111"));
    match s1 {
        Some(_x) => {
            println!("匹配上了 _x 但是没使用")
        }
        None => println!("no match"),
    }
    // 匹配上没使用，存在所有权转移
    // println!("s1 is {}", s1);
    let s2 = Some(String::from("222"));
    match s2 {
        Some(_) => {
            println!("match s2 do nothing")
        }
        None => println!("no match"),
    }
    // 忽略匹配，不存在所有权转移
    println!("s2 is {:?}", s2);

    // 忽略剩余部分
    let t1 = (1, 2, 3);
    let (t2, ..) = t1;
    let (t3, .., t4) = t1;
    // 报错
    // let (..,t5,..) =t1;
    println!("t2 is {t2}");

    struct Point {
        x: i32,
        y: i32,
    }
    let p1 = Point { x: 1, y: 2 };
    let Point { x: rename_x, .. } = p1;
    println!("rename_x is {rename_x}")
}
