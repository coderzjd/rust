fn main() {
    // 1、在函数前使用unfase声明 unsafe 函数
    unsafe fn unsafe_fn() {}
    // 2、unsafe函数需要在unsafe块或者unsafe函数内调用
    unsafe fn unsafe_bar() {
        unsafe_fn()
    }
    unsafe {
        unsafe_fn();
        unsafe_bar()
    }
}
