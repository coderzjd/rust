fn main() {
    let s1 = String::from("2333");
    {
        let s2 = String::from("998");
        let longer = get_longer(&s1, &s2);
        println!("longer is {}", longer)
    }
    // 这里的'a生命周期参数指定返回值的类型生命周期是s2和s3中较短的那一个
    fn get_longer<'a>(s1: &'a String, s2: &'a String) -> &'a String {
        if (s1.len() >= s2.len()) {
            return s1;
        } else {
            return s2;
        }
    }
    // 下面这种写法就会报错，因为s2的生命周期时间太短了，在外层scoped打印longer的时候s2已经失效
    // let longer;
    // {
    //     let s2 = String::from("998");
    //     // `s2` does not live long enough borrowed value does not live long enough
    //     longer = get_longer(&s1, &s2);
    // }
    // println!("longer is {}", longer)
}
