fn main() {
    // 基础数据类型是存放在stack上的
    // 1、标量类型(表示单一的值)
    // 1、整数类型 u表示无符号(表示非负整数)，i表示有符号
    // u8 u16 u32 u64 u128 usize(usize根据计算机位数有关，32为就是u32)
    // i8 i16 i32 i64 i128 isize(isize根据计算机位数有关，64为就是i64)
    // 默认为i32类型
    let num1 = 1;
    let num2: u8 = 1;
    let num3: u8 = 0;
    let num4: i32 = 0;
    // 默认为十进制
    let num5 = 10_000;
    // (可以使用_方便阅读)num5 is 10000
    println!("num5 is {num5}");
    // 十六进制
    let num6 = 0xff;
    println!("num6 is {num6}");
    // 八进制
    let num7 = 0o77;
    println!("num7 is {num7}");
    // 二进制
    let num8 = 0b1000;
    println!("num8 is {num8}");
    // Byte(字节)只能是u8类型
    let num9 = b'A';
    println!("num9 is {num9}");

    //      2、浮点类型
    // 1、f32(4字节)和f64(8字节)类型，都是有符号的
    // 默认为f64类型
    let num10 = 1.11;

    //      3、bool类型
    // 两个值true,false 占用一个字节
    let flag = true;
    let flag = false;
    println!("flag is {flag}");

    //      4、字符类型
    // char 使用单引号表示,占用4字节
    let char = 'A';
    println!("char is {char}");

    // 5、字符串字面量类型 &str,使用双引号表示，
    // 和js完全不一样，暂时还不是很理解(https://zhuanlan.zhihu.com/p/123278299)
    let info = "aaaaa";
    println!("{}", info);

    // 2、复合类型(多个类型的值组合在一起)
    // 数组和元组编译前必须知道大小(这里和js完全不一样)
    // 1、元组
    // 长度固定，元素类型固定，可以包含多种类型的值
    let tuple1 = (1, '2', 1.1);
    println!("tuple1.0 取值为： {}", tuple1.0);
    // 模式匹配来解构元组值
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    // 2、数组
    // 长度固定、元素类型相同
    let arr = [1, 2, 3];
    // 类型[i32;2]表示类型为i32长度为2的数组
    let arr2: [i32; 2] = [1, 2];
    // 赋值 [3;5]表示元素为3的长度为5的数组
    let arr3 = [3; 5];
    println!("arr3 is {:?}", arr3);
    println!("arr 第0个元素是： {}", arr[0]);
    // 模式匹配来解构数组
    let arr = [1, 2];
    let [a, b] = arr;
    println!("a is {},b is {}", a, b)
}
