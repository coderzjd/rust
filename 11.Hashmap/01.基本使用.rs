use std::collections::HashMap;

fn main() {
    // HashMap<K,V>
    // K值可以是任何类型,K是唯一的
    // 同一个hashmap的key的类型必须一致
    // 同一个hashmap的value的类型必须一致
    // 存储在heap上
    let mut m1: HashMap<String, i32> = HashMap::new();
    m1.insert(String::from("张三"), 18);
    m1.insert(String::from("李四"), 19);

    // 以下两种取值方法，效果一样
    let info = m1.get(&String::from("张三")).unwrap();
    println!("info is {info}");
    let lisi = String::from("李四");
    let lisi_info = m1.get(&lisi).unwrap();
    println!("lisi_info is {lisi_info}");

    // 遍历hashmap
    for (key, value) in &m1 {
        println!("key is {key} value is {value}")
    }

    let info_1 = String::from("2333");
    let info_2 = String::from("998");
    let mut m2: HashMap<String, String> = HashMap::new();
    m2.insert(info_1, info_2);
    // hashmap存储值的时候会触发所有权问题
    // println!("info_1 is {}",info_1);
    // println!("info_2 is {}",info_2);
}
