mod user {
    // 导出结构体需要声明字段为pub才能外部使用
    pub struct info {
        pub name: String,
        pub age: u8,
    }
    // 枚举类型字段没有限制
    pub enum Direction {
        Left,
        Right,
    }
}
fn main() {
    let u1 = user::info {
        name: String::from("张三"),
        age: 18,
    };
    let d1 = user::Direction::Left;
}
