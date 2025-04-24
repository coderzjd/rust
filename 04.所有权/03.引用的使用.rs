fn main() {
    // 引用的内存关系，相当于c语言的指针
    // String类型存储在heap上
    // 变量m1存储在stack上，保存的内存地址指向heap上的String
    let m1 = String::from("11");
    let m2 = String::from("22");
    println!("m1: {}", m1);
    println!(" m1 指向的内容地址: {:p}", &m1);
    greet(&m1, &m2);
    //  greet(&m1, &m2);对于存储在heap上的变量使用&引用之后不会获得m1,m2的所有权
    // 所有权还存在所以可以继续let s = format!("{},{}", m1, m2);
    let s = format!("{},{}", m1, m2);
}
fn greet(g1: &String, g2: &String) {
    println!("{},{}", g1, g2);
    let address_in_g1 = g1 as *const String;
    println!("{g1}");
    println!("g1存的内容：的地址{:p}", address_in_g1);
    println!("g1的地址: {:p}", &g1);
}
