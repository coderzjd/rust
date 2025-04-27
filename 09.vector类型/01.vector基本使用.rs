fn main() {
    // vector是连续存储的数据类型，存储数据类型一致
    // 基本数据类型的数组大小是不能变的
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let mut v3 = vec![1, 2, 3];
    v3.push(4);
    println!("v3 is {:?}", v3);

    // 获取vector值
    // 1、通过下标获取，可能存在下标越界触发runtime panic
    let p1 = &v3[1];
    println!("p1 is {}", p1);
    let p2 = v3.get(1);
    match p2 {
        Some(x) => {
            println!("p2 is {x}")
        }
        None => {
            println!("v3 not get index 2")
        }
    }

    let v4 = vec![1, 2, 3];
    let v4_1 = &v4[1];
    // 这里存在所有权问题，已经被借用为不可变引用了，push操作相当于可变引用
    // v4.push(4);
    // 是因为： vector在heap，每次push后大小改变重新分配内存地址，导致上面的不可变引用失效
    println!("v4 is {:?}", v4);

    let v5 = vec![1, 2, 3];
    for item in &v5 {
        let plus_one = *item + 1;
        println!(" plus_one is {plus_one}")
    }
    println!("v5 is {:?}", v5);

    let mut v6 = vec![1, 2, 3];
    for item in &mut v6 {
        *item += 1;
    }
    println!("v6 is {:?}", v6);
}
