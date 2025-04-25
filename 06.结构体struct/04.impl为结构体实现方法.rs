#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 使用impl为结构体实现方法
impl Rectangle {
    // 结构体方法的第一个参数必须为self或者&self
    fn area(&self) -> u32 {
        // &self时 self类型为&Rectangle，不回去所有权
        // let a: &Rectangle = self;
        // 下面两种写法都行
        // 1、第一种直接拿到width类型为u32
        let a = self.width;
        // self.width * self.height;
        // 第二种拿到width的引用： &u32 ,但是rust会自动解引用
        let b = &self.width;
        &self.width * &self.height
    }
    //  self时 self类型为Rectangle，获取所有权
    fn get_width(self) -> u32 {
        // let a: Rectangle = self;
        self.width
    }
    // 没有self或者&self参数的函数为关联函数
    // 大写的Self表达实现方法的类型，这里是Rectangle类型
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    // 使用mut self 可以获取所有权并修改
    fn bar(mut self) {
        self.width = 100;
        println!("self in bar  is {self:#?}");
    }
}
fn main() {
    let r1 = Rectangle {
        width: 10,
        height: 10,
    };
    println!("r1 is {r1:#?}");
    println!("r1 area: {}", r1.area());
    println!("r1 is {r1:#?}");
    let width = r1.get_width();
    println!("r1 width is {width}");
    // get_width函数获取了所有权，后续r1无法使用
    // println!("r1 is {r1:#?}");
    // 关联函数的调用： 直接使用结构体::方法名称
    let r2 = Rectangle::new(1, 10);
    println!("r2 is {r2:#?}");
    let widt_1 = Rectangle::area(&r2);

    let mut r3 = Rectangle::new(9, 9);
    fn bar(r: &mut Rectangle) {
        r.width = 10
    }
    bar(&mut r3);
    // println!("r3 is {r3:#?}");
    r3.bar();
}
