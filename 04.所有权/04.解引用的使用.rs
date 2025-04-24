// rust存在隐式的解引用
fn main() {
    // Box<T> 是一个指针，它指向堆上分配的类型为 T 的数据。当你创建一个 Box<T> 时，
    // Rust 会在堆上为数据分配空间，并将数据存储在堆上，而栈上只会存储指向堆数据的指针。
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;
    // 一层解引用拿到heap上的数据
    *x += 1;
    println!("x is {}", x); //此处rust隐式的解引用
    println!("a is {a}");

    // r1是一个(&Box<i32>)指针类型
    let r1: &Box<i32> = &x;
    // 二层解引用
    let b: i32 = **r1;
    println!("b is {}", b);
    // &*x 相当于先拿到 *x(i32),在进行& 取地址符r2,再使用*r2解引用拿到i32
    // 所以&*x相当于啥都没做，直接拿x
    let r2: &i32 = &*x;
    let c = *r2;
    println!("c is {}", c);

    let aa = Box::new(0);
    let bb = Box::new(&aa);
    let cc = ***bb;
    println!("bb is {}", bb)
}
