fn main() {
    let num: u8 = 4;
    let str: char = '中';
    let foo = num + str as u8;
    println!("num is {num},str is {str}");
    println!("foo is {foo}");

    const AAA: f32 = 3.4028236;
    // 浮点数精度问题
    println!("{AAA}");
    println!("{}", f32::MAX);

    let x: u8 = if true { 1 } else { 2 };
    println!("x is {x}");
    let x: u8;
    if bool::from(x) {
        x = 1;
    } else {
        x = 2
    }
    println!("x is {x}");
}
