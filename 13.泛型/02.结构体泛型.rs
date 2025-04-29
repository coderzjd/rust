fn main() {
    // rust没有继承
    struct Point<T, K> {
        x: T,
        y: K,
    }
    impl<T, K> Point<T, K> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    // 为具体的类型实现方法
    impl Point<i32, i32> {
        fn gkd(&self) -> &i32 {
            &self.x
        }
        // 不能为泛型类型和具体类型实现同一种方法x
        // fn x(){
        // }
    }
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: "1", y: "2" };
    let x = p1.x();
    println!("p1 x is {x}");
    // 为Point<i32, i32>类型实现了gkd方法
    // p1传参为i32类型，p1可以调用gkd方法
    p1.gkd();
    // p2类型不匹配，不行
    // p2.gkd();
}
