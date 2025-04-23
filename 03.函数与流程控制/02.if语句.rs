fn main() {
    // if的条件语句没有括号
    let num = 10;
    if num % 2 == 0 {
        println!("num 被 2整除")
    } else if num % 3 == 0 {
        println!("num 被 3整除")
    } else {
        println!("num 不能被 2,3整除")
    }
    // 条件语句必须为bool类型
    // if num {
    //     println!("111")
    // }

    // if表达式动态赋值
    let condition = true;
    // if分支的返回值类型必须一样
    // let number = if condition { 5 } else { "six" };
    let number = if condition { 5 } else { 6 };
}
