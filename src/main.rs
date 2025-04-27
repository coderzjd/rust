fn main() {
    // rust语言核心 str(&str)[相当于静态的，在编译时就确定下来]
    // String类型 来自标准库(可增长、用于所有权，utf8编码)

    // 1、创建字符串
    let s1 = String::new();
    let s2 = "111".to_string();
    let s3 = String::from("998");
    // 2、添加字符串
    let mut s4 = String::from("hellow");
    s4.push_str(" world");
    println!("s4 is {s4}");
    let mut s5 = String::from("lo");
    // 3、添加单个字符
    s5.push('l');
    println!("s5 is {s5}");

    let s6 = String::from("hellow");
    let s7 = String::from("_world");
    let s8 = s6 + &s7;
    // 这里会报错，使用【+】连接字符串相当于调用add方法,丢失了s6的所有权
    // fn add(self,s:&str)->String {}
    // println!("s6 is {s6}");
    println!("s8 is {s8}");

    // format!的使用：不丢失所有权
    let s9 = String::from("hellow");
    let s10 = String::from("_world");
    let s11 = format!("{s9}{s10}");
    println!("s11 is {s11}");
    println!("s9 is {s9}")
}
