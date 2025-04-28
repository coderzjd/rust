use std::collections::HashMap;

fn main() {
    let mut m1: HashMap<String, i32> = HashMap::new();
    m1.insert(String::from("李四"), 19);
    // 1、插入相同的Key会直接替换
    m1.insert(String::from("张三"), 18);
    m1.insert(String::from("张三"), 18);
    println!("m1 is {:?}", m1);
    // 2、查看王五是否存在，如果不存在新增key为王五,value为20
    m1.entry(String::from("王五")).or_insert(20);
    // 3、因为李四存在，所以不修改里面的值
    m1.entry(String::from("李四")).or_insert(20);
    println!("m1 is {:?}", m1);

    // 统计单词次数
    let text = "hellow world wonderful world";
    let mut m2 = HashMap::new();
    for word in text.split_whitespace() {
        // 这里or_insert返回了值的可变引用，使用解引用直接修改
        let count = m2.entry(word).or_insert(0);
        *count += 1
    }
    print!("m2 is {m2:?}")
}
