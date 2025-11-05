fn main() {
    // print输出不换行
    print!("1");
    print!("1");
    // println输出会换行（带有换行符输出）
    println!("1");
    println!("1");

    let x = 42;
    // {:p}只能对指针类型进行格式化输出
    // * 在这段代码中，`&x` 是变量`x`的引用，即指向`x`的指针。`{:p}`会使`println!`宏以十六进制形式输出这个指针所指向的内存地址。
    println!("The address of x is: {:p}", &x);
    // println!("{}", 42); 输出42（整数值）
    // * println!("{:p}", &42); 输出类似0x7ffd3973abdc（内存地址，具体值取决于运行环境

    // println和print输出都会解引用
    println!("The address of x is: {}", &x);
}
