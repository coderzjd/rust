fn main() {
    // 函数指针
    // 1、所以函数都会转化为fn类型
    // 2、函数指针实现了三种闭包特性：FN,FnMut,FnOnce
    // 3、可以在需要闭包的地方使用函数

    // 类似JS，函数为一等公民。可以作为参数.但是不能作为返回值
    type AddFn = fn(a: i32, b: i32) -> i32;
    fn foo(founction: AddFn, num: i32) -> i32 {
        founction(num, num)
    }
    fn bar(n: i32, m: i32) -> i32 {
        m + n
    }
    let res = foo(bar, 1);
    println!("res is {res}");
    
    // 使用Box<T>类型包装闭包实现函数返回值
    type AddN = Box<dyn Fn(i32) -> i32>;

    fn addn(n: i32) -> AddN {
        let aa = move |x: i32| -> i32 { n + x };
        Box::new(aa)
    }
    let add5 = addn(5);
    let rsp = (*add5)(1);
    println!("rsp is {rsp}")
}
