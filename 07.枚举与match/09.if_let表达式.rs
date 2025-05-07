fn main() {
    let info = Some(String::from("2333"));
    match info {
        Some(x) => {
            println!("match some info is {}", x)
        }
        _ => {}
    }
    let info2 = Some(String::from("998"));
    // if let 等价于上面的match表达式
    if let Some(x) = info2 {
        println!("if let x is {}", x)
    }
    // if let只会处理匹配上的，未匹配默认不处理
    // 需要处理加上else语句
    let info3 = Some(String::from("998"));
    if let Some(x) = info3 {
        println!("if let x is {}", x)
    } else {
        println!("none")
    }
    
    // if let表达式匹配多个
    let tuple_info = (Some(1), Some(2));
    if let (Some(x), Some(y)) = tuple_info {
        println!("x is {x}");
        println!("y is {y}");
    }
}
