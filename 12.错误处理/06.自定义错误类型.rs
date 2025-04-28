use std::{
    fs::File,
    io::{self, Read},
    num::ParseIntError,
};

fn main() {
    // ?操作符会隐式调用from这个trait
    // 从其他类型的Error转化为MyErr
    enum MyErr {
        Io(io::Error),
        ParseInt(ParseIntError),
        Other(String),
    }
    impl From<io::Error> for MyErr {
        fn from(value: io::Error) -> Self {
            Self::Io(value)
        }
    }
    // 为自定义错误MyErr实现了from这个trait,后续就可以抛出
    impl From<ParseIntError> for MyErr {
        fn from(value: ParseIntError) -> Self {
            Self::ParseInt(value)
        }
    }
    fn throw_myerr() -> Result<String, MyErr> {
        let mut s1 = String::new();
        File::open("aaa.txt")?.read_to_string(&mut s1)?;
        let num: i32 = "55".parse()?;
        Ok(s1)
    }
}
