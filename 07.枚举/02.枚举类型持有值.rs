fn main() {
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let four: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
    let six: IpAddrKind = IpAddrKind::V6(String::from("::1"));
    let five: fn(u8, u8, u8, u8) -> IpAddrKind = IpAddrKind::V4;
    let all: IpAddrKind = five(1, 2, 3, 4);
}
