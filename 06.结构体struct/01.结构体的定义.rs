fn main() {
    // 结构体存储在heap上
    // 得到debug宏
    #[derive(Debug)]
    // struct关键字定义结构体
    struct User {
        name: String,
        age: u8,
        height: f32,
    }
    let age = 19;
    let zhangsan = String::from("张三");
    let u1 = User {
        // 生成结构体类似js可以简写同名key-value
        age,
        height: 1.77,
        name: zhangsan,
    };
    // zhangsan 变量被赋值到结构体相当于变量赋值，存在所有权转移
    // println!("zhangsan name is {}",zhangsan);
    println!("u1 is {:?}", u1);
    println!("u1 name: {}", u1.name);
    println!("u1 age: {}", u1.age);
    println!("u1 height: {}", u1.height);
    // 使用..对结构体变量进行复用，这里和js不一样
    // 1、复用结构体放在后面
    // 2、使用【..】操作符
    // 3、结尾不跟着逗号
    let u2 = User {
        name: String::from("李四"),
        ..u1
    };
    // u1在解构到u3时age、height段不存在所有权转移，可以继续使用
    println!("u1 is {:?}", u1);

    println!("u2 is {:?}", u2);
    println!("u2 name: {}", u2.name);
    println!("u2 age: {}", u2.age);
    println!("u2 height: {}", u2.height);

    let u3 = User {
        age: 88,
        height: 2.1,
        ..u1
    };
    println!("u3 is {:?}", u3);
    // 部分移动值的借用报错
    // u1在解构到u3时name字段存在所有权转移，无法使用
    // println!("u1 is {:?}", u1);
    // u1在解构到u3时age、height段不存在所有权转移，可以继续使用
    println!("u1 age: {}", u1.age);
    println!("u1 height: {}", u1.height);

    // 修改变量的值
    let mut u4 = User { ..u2 };
    println!("u4 age: {}", u4.age);
    u4.age = 29;
    println!("u4 age: {}", u4.age);

    // 这里写错，生成了Range<User>类型，不知道为什么？
    let u5 = { u3..u4 };
    print!("{:?}", u5.start);
    print!("{:?}", u5.end)
}
