fn main() {
    // 1、原始指针【*是类型的一部分,不代表解引用】
    // *const T : 不可变原始指针
    // *mut T   : 可变原始指针
    // 1、可以忽略借用规则（存在多个可变指针）
    // 2、不保证指向有效内存
    // 3、可以为null
    // 4、不会自动清理（没用drop）

    let mut num = 1;
    // 可以在safe代码中创建原始指针
    let r1 = &raw const num;
    let r2 = &raw mut num;
    println!("r1 is {:?}", r1);
    // 原始指针的解引用只能放在unsafe块中进行
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
        *r2 += 2;
        println!("r2 is {}", *r2);
    }
}
