fn main() {
    println!("hello rust");
    foo();
    bar();
    dyn_gkd()
}
// 泛型 trait 用 一份通用源码 换 N 份机器码——省事，适合“同逻辑无限类型”（From<T>、Add<RHS>）。
fn foo() {
    trait Gen<T> {
        fn go(&self) -> T;
    }

    struct Plant;
    // 只写一次，下面再调用 Plant::go::<String> 和 Plant::go::<u8>
    impl<T: Default> Gen<T> for Plant {
        fn go(&self) -> T {
            T::default()
        }
    }

    fn use_it() {
        let _: String = Plant.go(); // 自动生成 Gen<String>
        let _: u8 = Plant.go(); // 自动生成 Gen<u8>
    }
    use_it()
}
// 关联类型 用 N 份手写 impl 换 N 份机器码——可控，适合“输出类型唯一”（Iterator::Item、Service::Error）。
fn bar() {
    trait Assoc {
        type Item;
        fn go(&self) -> Self::Item;
    }

    struct PlantS;
    struct PlantU;

    impl Assoc for PlantS {
        type Item = String;
        fn go(&self) -> String {
            "a".into()
        }
    }
    impl Assoc for PlantU {
        type Item = u8;
        fn go(&self) -> u8 {
            42
        }
    }

    fn use_it() {
        let _: String = PlantS.go();
        let _: u8 = PlantU.go();
    }
    use_it()
}
//  动态分发 dyn Trait
fn dyn_gkd() {
    trait Draw {
        fn draw(&self);
    }
    struct  Circle;
    struct  Square;
    impl Draw for Circle {
        fn draw(&self) {
            println!("○");
        }
    }
    impl Draw for Square {
        fn draw(&self) {
            println!("□");
        }
    }

    fn paint(shape: &dyn Draw) {
        shape.draw();
    } // ← 这里发生动态分发

    fn some() {
        let c = Circle;
        let s = Square;
        paint(&c as &dyn Draw);
        paint(&s as &dyn Draw);
    }
    some()
}
// 泛型与关联类型比较
// 不论哪种写法，最后只要用到的具体类型不同，都会单态化——都会各生成一份机器码。差别只在“谁写 impl”：
// 泛型参数 → 一份通用源码，编译器帮你自动扩写 N 份 impl。
// 关联类型 → 你得手动写 N 份 impl，源码层面没有复用。
// 所以性能上没差距（最终都是 N 份机器码），差距在写代码的省事程度！
// 使用技巧
// 如果 “这个 trait 对于某个类型只能有一种形态” → 关联类型
// 如果 “调用方希望多次用不同类型参数重复实现这个 trait” → 泛型参数

// 动态分发 dyn Trait
// 只生成一份 paint 机器码。
// 运行时 shape 变成 胖指针（2 × usize）：
// data 指针 → 实际对象
// vtable 指针 → 虚函数表（里面存着真正 draw 的地址）
// 调用 shape.draw() 时 CPU 先 load vtable → 再 load 函数地址 → 再 jump。
// 多了一次间接跳转 + 无法内联，因此有微小运行时开销

// 泛型 trait” 编译期展开，跑得快体积大；
// dyn Trait 运行时查表，体积小点稍慢；
// 业务里性能路径优先泛型，插件/对象池/事件回调再掏 dyn