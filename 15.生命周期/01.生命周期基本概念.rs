fn main() {
    // 防止悬垂引用
    // 生命周期：确保引用在所需的时间内有效（引用的有效范围Scope）
    // 每一个引用都有生命周期
    // 大多数时候生命周期是隐式的，且可以被推断出来
    fn life_times<'a>(s: &'a String) {
        println!("s is {s}")
    }
    let s1 = String::from("hellow_life_times");
    life_times(&s1);
    // 当引用的生命周期可能以不同的方式相关联时需要主动标注生命周期

    // 借用检查器：确保数据存活的时间长于其引用
    // 生命周期注解：
    // 不改变引用存活时间
    // 展示描述多个引用之间的生命周期关系

    let s2 = String::from("2333");
    let s3 = String::from("998");
    // 这里的'a生命周期参数指定返回值的类型生命周期是s2和s3中较短的那一个
    fn get_longer<'a>(s1: &'a String, s2: &'a String) -> &'a String {
        if (s1.len() >= s2.len()) {
            return s1;
        } else {
            return s2;
        }
    }
    // 借用检查器通过生命周期参数检查s1和s2的生命周期是否可以覆盖返回值long这个引用的作用域[比他长就行]
    let long = get_longer(&s2, &s3);
    println!("long is {}", long);
}
