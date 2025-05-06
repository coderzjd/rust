fn main() {
    // 迭代器是惰性的，不消耗相当于没写
    let arr = vec![1, 2, 3];
    for item in arr.iter() {
        println!("item is {item}")
    }
    // 所有的迭代器都实现了标准库的iterator Trait
    // pub trait Iterator {
    //      定义关联类型
    //     type Item;
    //     fn next(&mut self)->Option<Self::Item>;
    // }

    // 迭代器的next方法，有值则使用Some包裹，没有值、迭代结束就返回None【和js类似】
    let arr1 = vec![1, 2, 3];
    let mut iterator = arr.iter();
    let info = iterator.next();
    let info = iterator.next();
    let info = iterator.next();
    let info = iterator.next();

    // 迭代器：
    // 1、iter()   遍历不可变引用
    // 2、into_iter() 可以获得所有权，并且返回具有所有权的值
    // 3、iter_mut() 遍历可变引用

    let arr3 = vec![String::from("1"), String::from("2"), String::from("3")];
    for item in arr3.into_iter() {
        println!("item is {item}")
    }
    // into_iter获取所有权，后续会报错
    // println!("item is {arr3:?}")

    let mut arr4 = vec![1, 2, 3];
    for item in arr4.iter_mut() {
        *item += 1
    }
    println!("arr4 is {arr4:?}");

    // 内置方法消耗迭代器
    let arr5 = vec![1, 2, 3];
    let total: i32 = arr5.iter().sum();
    println!("total is {total}");
    // 迭代器适配器
    let arr5 = vec![1, 2, 3];
    // map方法不消耗迭代器，产生新的迭代器
    let info: Vec<i32> = arr5.iter().map(|x| 2 * x).collect();
    println!("info is {info:?}");

    let arr6 = vec![1, 2, 3];
    let filter_info: Vec<i32> = arr6.into_iter().filter(|x| *x % 2 == 0).collect();
    println!("filter_info is {filter_info:?}")
}
