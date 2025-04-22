use rand;
use std::{cmp::Ordering, io::stdin};
fn main() {
    let num = rand::random_range(1..=10);
    println!("生成的随机数是{num}");
    let mut msg = String::new();
    loop {
        msg.clear();
        println!("请输入一个数字");
        stdin().read_line(&mut msg).unwrap();
        let user_input: i32 = match msg.trim().parse() {
            Ok(res) => res,
            _ => {
                println!("错误：无法解析为数字");
                continue;
            }
        };
        match num.cmp(&user_input) {
            Ordering::Equal => {
                println!("恭喜你猜对啦");
                break;
            }
            Ordering::Less => {
                println!("数字偏大")
            }
            Ordering::Greater => {
                println!("数字偏小")
            }
        }
    }
}
