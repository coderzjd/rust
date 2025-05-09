fn main() {
    // 多模式匹配
    let x = Some(2);
    match x {
        Some(1) => println!("1 is match "),
        Some(x) => println!("x is macth {x}"),
        None => println!("match None"),
    }

    let x = 4;
    match x {
        1 | 2 => println!("1 or 2 is match "),
        3 => println!("3 is match"),
        _x => println!("match other {_x}"),
    }

    // 匹配范围[只允许整数和char]
    let y = 2;
    match y {
        1..5 => println!("y in 1..5"),
        _x => println!("match otehr {_x}"),
    }
    let c = 'k';
    match c {
        'a'..='c' => println!("c in 'a'..='c'"),
        _x => println!("match otehr {_x}"),
    }

    // 结构体匹配
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p1 = Point { x: 0, y: 2 };
    match p1 {
        Point { x: 0, y } => println!("点在 y 轴上"),
        Point { x, y: 0 } => println!("点在 x 轴上"),
        Point { x, y } => println!("点坐标 x is {x}, y is {y}"),
    }

    // 匹配枚举
    enum Animal {
        Cat(i32),
        Dog,
    }
    let a1 = Animal::Cat(1);
    match a1 {
        Animal::Cat(x) => println!("match cat x is {x}"),
        Animal::Dog => println!("match Dog"),
    }
}
