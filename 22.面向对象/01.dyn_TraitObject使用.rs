fn main() {
    // Trait Object
    // 1、rust提供泛型支持类型抽象、但是泛型要求类型在编译时已知
    // 2、Trait Object 是另一种抽象、在运行时支持不同类型、前提是他们实现了某个trait
    // 3、类似与OOP中的"接口+多态"或者TS中的鸭子类型

    trait Draw {
        fn draw(&self);
    }
    struct Button {
        width: i32,
        height: i32,
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("draw button")
        }
    }
    // 这里限制了Box类型必须为实现了Draw类型的子类
    let arr: Vec<Box<dyn Draw>> = vec![
        Box::new(Button {
            width: 1,
            height: 2,
        }),
        // 报错,String类型没有实现Draw
        // Box::new(String),
    ];
    for item in arr.iter() {
        item.draw();
    }

    // 与泛型的区别
    // 1、泛型具有单态化，在编译时确定具体类型，在运行时只能有特定的一种类型
    // 2、Trait Object在运行时派发，可以有多种类型
    // 3、泛型编译时优化性能更强、Trait Object有多余的开销
}
