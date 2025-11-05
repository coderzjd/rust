fn main() {
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let four: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
    let six: IpAddrKind = IpAddrKind::V6(String::from("::1"));
    // 枚举构造器天然就是函数指针
    let five: fn(u8, u8, u8, u8) -> IpAddrKind = IpAddrKind::V4;
    // 等价与下面写法
    // fn V4(a: u8, b: u8, c: u8, d: u8) -> IpAddrKind {
    //     IpAddrKind::V4(a, b, c, d)
    // }
    let all: IpAddrKind = five(1, 2, 3, 4);

}
