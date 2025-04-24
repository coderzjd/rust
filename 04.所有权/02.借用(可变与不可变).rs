// 借用不该变所有权
// 借用包含：可变引用，不可变引用
fn main() {
    let str = String::from("hellow");
    // foo(str);
    // borrow of moved value: `str` value borrowed here after move
    // 基于所有权规则，当前str在foo传参时，str已经丢失所有权，后续无法使用
    // println!("str is {}", str);

    bar(&str);
    // bar函数使用借用，借用中的不可变引用
    // 在bar函数传参之后不影响str的所有权
    println!("str is {}", str);

    let mut str1 = String::from("2333");
    // gkd函数使用借用，借用中的可变引用
    // 在gkd函数传参之后不影响str1的所有权,并且修改了str1的值
    gkd(&mut str1);
    println!("str1 is {}", str1)
}
fn foo(str: String) {
    println!("foo str len : {}", str.len())
}

fn bar(str: &String) {
    println!("bar str len : {}", str.len())
}

fn gkd(str: &mut String) {
    str.push_str("998");
    println!("gkd str is {}", str)
}
