fn main() {
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
    ];
}
