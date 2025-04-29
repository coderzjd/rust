pub fn add(n: i32, m: i32) -> i32 {
    n + m
}
pub fn greater_3(num: i32) -> bool {
    num >= 3
}
// 单元测试用于测试内部代码，私有方法
// 使用cargo test运行所有单元测试
// 使用cargo test test_add运行名称为test_add的单元测试
#[cfg(test)] //标记改单元测试只在cargo test命令下运行，在build下不允许，不会编译到最终结果
mod tests {
    use super::*; //导入被测试方法
    #[test] //声明测试函数
    #[ignore] //声明调过此条单元测试[使用 cargo test -- --ignored 只运行被标记为ignore的测试]
    fn test_add() {
        let res = add(1, 2);
        // assert_eq! 断言结果相等，失败会打印比较的值,相当于调用==
        // assert_ne! 断言结果不相等，失败会打印比较的值，相当于调用!=
        // 被比较的值需要实现Debug和PartialEq这两个trait
        assert_eq!(res, 3);
        // 后面的参数用于定义失败信息
        assert_eq!(res, 4, "失败的信息 {}", res);
        assert_ne!(res, 4, "失败的信息 {}", res);
        assert!(greater_3(10), "失败的信息");
    }
    #[test]
    fn test_add_01() {
        let res = add(1, 2);
        assert_eq!(res, 3);
    }
    #[test]
    fn throw_panic() {
        // panic!主动触发测试函数失败
        panic!("throw_panic fail")
    }
    #[test]
    fn test_greater_3() {
        // assert! 断言资源是否为true
        assert!(greater_3(10));
    }
    #[test]
    #[should_panic] //断言函数应该触发panic!
    fn get_panic() {
        panic!("断言函数触发panic!")
    }
    #[test]
    // 用于匹配可能发生多种类型painc的一种
    #[should_panic(expected = "111")] //断言函数应该触发panic!，并且报错信息包含111
    fn get_panic_match_info() {
        panic!("因为panic信息包含expected的字段：111，所以测试通过")
    }
    #[test]
    #[should_panic(expected = "222")]
    fn get_panic_not_match_info() {
        panic!("因为panic信息不包含expected的字段，所以测试不通过")
    }
    #[test]
    // 返回类型为Result，可以用?测试返回Resultl的函数
    fn test_result_option() -> Result<(), String> {
        let res = add(1, 2);
        if res == 4 {
            Ok(())
        } else {
            Err(String::from("res不等于4"))
        }
    }
}
