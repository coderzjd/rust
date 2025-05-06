fn main() {
    // clone_str函数返回了一个闭包，需要声明闭包的生命周期，告诉编译器：返回的闭包的生命周期不能比s的生命周期长
    fn clone_str<'a>(s: &'a str) -> impl Fn() -> String + 'a {
        move || s.to_string()
    }
    // 可以简写为
    fn clone_str2(s: &str) -> impl Fn() -> String + '_ {
        move || s.to_string()
    }
    let s1 = String::from("hellow world");
    let cloner = clone_str(&s1);
    // 销毁s1会导致cloner闭包引起悬垂引用报错
    // drop(s1);
    cloner();
}
