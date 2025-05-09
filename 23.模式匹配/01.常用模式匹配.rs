fn main() {
    // 模式匹配
    // match匹配
    // 1、需要穷尽所有分支
    // 2、省略的分支使用下划线【_】代替

    // if let 匹配
    // 1、只会处理匹配上的情况
    // 2、没有匹配的在else处理
    let s1: Result<i32, ()> = Ok(1);
    if let Ok(5) = s1 {
        println!("5 is match")
    } else if let Ok(x) = s1 {
        println!("x is match {}", x)
    } else {
        println!("no match")
    }

    // while let 循环匹配 和if let很类似
    let v = vec![Some(1), Some(2), Some(3)];
    let mut iter = v.into_iter();
    while let Some(x) = iter.next() {
        match x {
            Some(value) => println!("{}", value),
            None => println!("None"),
        }
    }

    // for 循环匹配
    let v1 = vec![1, 2, 3];
    for (index, num) in v1.iter().enumerate() {
        println!("index is {}, num is {}", index, num)
    }
    //赋值匹配
    // 1、元组解构
    let a = (1, 2, 3);
    let (b, c, d) = a;

    // 2、函数传参解构
    fn foo((x, y): (i32, i32)) {
        println!("match x");
        println!("match y")
    }
    // 3、结构体解构,解构重命名
    struct Point {
        x: i32,
        y: i32,
    }
    let p1 = Point { x: 1, y: 2 };
    let Point { x, y } = p1;
    println!("x match {}, y match {}", x, y);
    let Point { x: a, y: b } = p1;
    println!("a match {}, b match {}", a, b);

    // 解构元组和结构体
    let b = ((1, 2), p1);
    let ((x, y), Point1 { x: xx, y: yy }) = b;
}
