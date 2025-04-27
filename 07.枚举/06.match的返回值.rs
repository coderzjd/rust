fn main() {
    let a = 1;
    let b = Some(1);
    // Option类型直接相加会报错，相当于TS的类型不匹配
    // let c = a + b;
    let c = match b {
        Some(x) => x,
        None => 0,
    };
    let d = a + c;
}
