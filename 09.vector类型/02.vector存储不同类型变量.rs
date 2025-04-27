fn main() {
    enum info {
        Text(String),
        Num(u8),
    }
    let v1 = vec![info::Text(String::from("张三")), info::Num(18)];
}
