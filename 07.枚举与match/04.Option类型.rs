fn main() {
    // rust内置枚举Option类型签名
    // enmu Option<T> {
    //     None,
    //     Some(T)
    // }
    // 表示某个值可能不存在
    let p1 = Option::Some(1);
    // 赋值为None，需要主动声明类型
    let p2: Option<u32> = None;
    match_option(p1);
    match_option(p2);
    fn match_option(p: Option<u32>) {
        match p {
            Some(x) => {
                println!("match Some is {x}")
            }
            None => {
                println!("match None")
            }
        }
    }
}
