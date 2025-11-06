macro_rules! log_all {
    // 单元素：打印 + 换行
    ($head:expr) => {
        println!("{}", $head);
    };
    // 多元素：打印当前，再递归剩余
    ($head:expr, $($tail:expr),+) => {
        println!("{}", $head);
        log_all!($($tail),+);
    };
}
fn main() {
    log_all!(1); // 1 个
    log_all!("hi", 3.14, true); // 3 个
    log_all!('a', 42, "rust", 9.9, false); // 5 个
}
