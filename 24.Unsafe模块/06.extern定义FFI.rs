fn main() {
    // extern 调用外部代码
    // 1、外部函数接口FFI
    // 2、Rust需要与其他语言进行交互
    // 3、extern关键字创建和使用外部语言接口FFI
     unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("extern C {}", abs(-2));
    }
}

