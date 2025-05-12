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

    // 1、foo和bar函数都返回Result<T>，但是略有不同
    fn foo() -> Result<String, ()> {
        Ok(String::from("988"))
    }
    fn bar() -> Result<i32, ()> {
        Ok(2)
    }
    // 2、通过类型别名+泛型参数优化
    type myResult<T> = Result<T, ()>;
    fn foo1() -> myResult<String> {
        Ok(String::from("988"))
    }
    fn bar1() -> myResult<i32> {
        Ok(2)
    }
}
