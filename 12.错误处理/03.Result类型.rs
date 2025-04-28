use std::{fs::File, io::Read};

fn main() {
    // Result枚举类型
    // Ok(),Err()
    let file: Result<File, std::io::Error> = File::open("aaa.txt");
    match file {
        // 可以修改Ok()类型变量的可变性
        Ok(mut res) => {
            // res为文件句柄，可以访问文件
            println!("file is {:?}", res); 
            let mut text = String::new();
            res.read_to_string(&mut text).unwrap();
            println!("aaa.txt read is {}", text)
        }
        Err(e) => {
            println!("err is {}", e)
        }
    }
}
