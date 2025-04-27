fn main() {
    let arr = [3; 5];
    for item in arr {
        println!("item is {}", item)
    }
    println!("{}", arr.len());
    let arr = arr.into_iter().enumerate();
    for (item, index) in arr {
        println!("item is {},index is {}", item, index)
    }
    for item in 1..=4 {
        println!("item1=4  is {}", item)
    }
    for item in 1..4 {
        println!("item1..4 is {}", item)
    }

    // 注意for循环的数据类型
    let arr1 = [3; 5];
    for item in arr1 {
        // 这里的 item 是i32类型
        println!("item is {}", item)
    }
    for item in &arr1 {
        // 这里的 item 是&i32类型
        println!("item is {}", item)
    }
}
