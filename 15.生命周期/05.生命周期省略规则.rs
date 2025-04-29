fn main() {
    // 1、默认情况下需要给所有引用标注生命周期
    // 过于繁琐，产出三条生命周期省略规则，如果这三条规则不满足就报错，需要主动声明生命周期了
    // 1、编译器为每个引用类型参数分配不同的生命周期参数
    // 2、如果只有一个声明之前输入参数、改生命周期将分配给所有输出参数
    // 3、如果有多个引用参数，其中一个是&self或者&mut self，那么把self的生命周期参数分配给所有的输出参数

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn foo<'b: 'a>(&self, s: &'b str) -> &'b str {
            s
        }
    }
}
