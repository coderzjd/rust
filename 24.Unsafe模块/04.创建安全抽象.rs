use std::slice;

fn main() {
    let mut v1 = vec![1, 2, 3, 4, 5];
    let r1 = &mut v1[..];
    let info = split_at_mut(r1, 3);
    println!("info is {info:?}")
}
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    // 获取可变原始指针
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    // 创建安全抽象
    // 在安全代码中使用unsafe块调用不安全代码
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
