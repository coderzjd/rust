use std::fs::File;

fn main() {
    // unwrap：出现None或者Err错误直接使用panic!处理
    // let file = File::open("aaa.txt").unwrap();
    // expect: 与unwrap类似，展示新增了错误提示
    let file = File::open("aaa.txt").expect("文件不存在，打开失败！");
}
