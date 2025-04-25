fn main() {
    let mut string = String::from("123456789");
    let s1 = &string[0..1];
    println!("si is {}", s1);
    let s2 = &string[0..=1];
    // 字符串切片是一种引用，存在所有权问题
    // string.push_str("10");
    println!("si is {}", s2);
    // 在字符串切片使用完毕之后恢复权限
    string.push_str("10");
    println!(" string is {}", string);
    let s3 = &string[..string.len()];
    println!("s3 is {}", s3);

    let s4 = &string[..];
    println!("s4 is {}", s4);
    let s5 = &string[..3];
    println!("s5 is {}", s5);
    let s6 = &string[..=3];
    println!("s6 is {}", s6);
    // index 100 is out of bounds of `12345678910`
    // let s7 = &string[..100];
    // println!("s7 is {}", s7);
    foo("1");
    foo(&"1");
    foo(&string);
    fn foo(s: &str) {
        println!("foo s is {s}")
    }
}

