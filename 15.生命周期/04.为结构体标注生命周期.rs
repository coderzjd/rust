fn main() {
    // 这个结构体的part字段为引用类型，需要声明周期参数（感觉类似于函数调用）
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let s1 = String::from("2333");
    let s2 = &s1[..];
    let p1 = ImportantExcerpt { part: s2 };
    // drop(s1);
    println!("p1 part is {:?}", p1.part);

    impl<'a> ImportantExcerpt<'a> {
        fn foo(&self, s: &str) -> &str {
            &self.part
        }
    }
}
