use std::slice;

fn main() {
    // 随意访问内存地址，导致未定义的行为
    let address = 0x01234usize;
    let r = address as *mut i32;
    let value = unsafe { slice::from_raw_parts_mut(r, 100) };
    print!("{:?}", value)
}
