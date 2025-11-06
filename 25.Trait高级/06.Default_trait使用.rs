fn main() {
    println!("hello rust");
    let info = u8::default();
    println!("info is {}", info);
    let cfg = Config::default();
    println!("cfg' port  is {}", cfg.port);
    println!("cfg' host  is {}", cfg.host);
    println!(
        "Info's default is {:?},{}",
        Info::default(),
        Info::default().age
    )
}
#[derive(Default, Debug)]
struct Config {
    port: u16,
    host: String,
}
#[derive(Debug)]
struct Info {
    age: u8,
}
impl Default for Info {
    fn default() -> Self {
        Info { age: 18 }
    }
}
// Default 简约实现
//  trait Default {
//     fn default() -> Self;
// }
// 相当于把golang中的默认初始化抽象到一个default trait默认调用也可以手动为struct实现
