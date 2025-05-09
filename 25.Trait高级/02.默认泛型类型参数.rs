use std::ops::Add;

fn main() {
    //  默认泛型类型参数
    // 1、拓展trait而不破坏代码
    // 2、运行特定情况下的自定义

    #[derive(Debug)]
    struct Point<T = i32> {
        width: T,
    }
    let p1 = Point { width: 1 };
    let p1 = Point {
        width: String::from("998"),
    };

    // 内置 std::ops::Add trait 实现
    // Rhs表示默认类型参数
    // trait Add<Rhs = Self> {
    //     type Output;
    //     fn add(self,rhs:Rhs)->Self::Output;
    // }
    impl Add for Point {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Point {
                width: self.width + rhs.width,
            }
        }
    }
    let p2 = Point { width: 2 };
    let p3 = Point { width: 3 };
    let p4 = p2 + p3;
    println!("p4 is {:?}", p4)
}
