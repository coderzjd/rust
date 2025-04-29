use test01::{Dog, Run};
fn main() {
    // 如果实例想使用trait方法，必须导入对应的trait
    let d1 = Dog {
        name: String::from("旺财"),
    };
    d1.run()
}
