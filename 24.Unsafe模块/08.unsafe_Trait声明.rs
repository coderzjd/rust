fn main() {
    // unsafe trait
    // 1、可以使用unsafe实现不安全trait
    // 2、当trait中至少有一个方法具有编译器无法验证的不变量时，改trait就是不安全的
    // 声明 unsafe trait
    unsafe trait Foo {}
    unsafe impl Foo for i32 {}
}
