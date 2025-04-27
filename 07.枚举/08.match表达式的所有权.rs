fn main() {
    let info = Some(String::from("2333"));
    match info {
        Some(x) => {
            println!("match sone x is {x}");
        }
        None => {
            println!("match none")
        }
    }
    // match表达式相当于函数调用，存在所有权转移
    // println!("info is {:?}", info)
    let info1 = Some(String::from("2333"));
    match info1 {
        // 使用下划线匹配，没有拿到值，不涉及所有权转移
        Some(_) => {
            println!("match sone not use ");
        }
        None => {
            println!("match none")
        }
    }
    println!("info1 is {:?}", info1);

    // match相当于函数可以使用借用，不影响所有权
    let info2 = Some(String::from("998"));
    match &info2 {
        Some(x) => {
            println!("match sone x is {x}");
        }
        None => {
            println!("match none")
        }
    }
    println!("info2 is {:?}", info2);
}
