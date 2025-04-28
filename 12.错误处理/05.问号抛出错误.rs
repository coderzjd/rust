use std::{
    fs::{self, File},
    io::{self, Read},
};

fn main() {
    // 主动抛出错误
    fn read_file() -> Result<String, std::io::Error> {
        let info = File::open("aaa.txt");
        let mut info_handler = match info {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut s1 = String::new();
        match info_handler.read_to_string(&mut s1) {
            Ok(_) => Ok(s1),
            Err(e) => Err(e),
        }
    }
    // 上下两种写法等价
    // 使用【?】简化抛出错误
    // 1、【?】必须在返回Result<T,Err>或者Option<T，K>类型或者实现了FromResidual类型函数里面
    fn read_file1() -> Result<String, io::Error> {
        let mut info = File::open("aaa.txt")?;
        let mut s1 = String::new();
        info.read_to_string(&mut s1)?;
        Ok(s1)
    }
    // 使用链式调用简化
    fn read_file2() -> Result<String, io::Error> {
        let mut s1 = String::new();
        File::open("aaa.txt")?.read_to_string(&mut s1)?;
        Ok(s1)
    }
    // 使用内置方法
    fn read_file3() -> Result<String, io::Error> {
        fs::read_to_string("aaa.txt")
    }
}
