fn main() {
    // 类型别名
    // 使用type关键字定义类型别名(和TS类似)
    type MyNum = u32;
    // MyNum 类型和u32相同
    let a: MyNum = 7;
    let b: MyNum = a + 1;
    println!("{b}");
    // 和TS不一样：不能定义字面量类型
    // type foo = 1 ;
}
