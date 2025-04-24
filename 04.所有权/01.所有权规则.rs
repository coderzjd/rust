fn main() {
    // 所有权规则：对于存储在heap上的数据， 基础数据存储在stack上
    // 首先，我们来看一下所有权规则。在我们通过示例来说明这些规则时，请牢记这些规则：
    // Rust 中的每个值都有一个所有者 。
    // 一次只能有一个所有者。
    // 当所有者超出范围时，该值将被删除。
    let a = Box::new(3);
    let b = a;
    // 把a赋值给b，b获得所有权，a会失效
    // println!("{}", a);

    // 对于没有实现copy trait 的数据类型，s1会丢失所有权
    let s1 = String::from("111");
    let s2 = s1;
    // println!("{}", s1);

    let s3 = String::from("111");
    // clone()会在产生新的数据，s3不涉及所有权转移
    let s4 = s3.clone();
    println!("{}", s3);
    // 对于实现copy trait 的数据类型会直接复制，不会丢失所有权
    let num1 = 1;
    let num2 = num1;
    println!("num2 is {num2}");

    // 函数传参相当于赋值语句会有所有权转移
    let str = "111".to_string();
    fn foo(s: String) {
        println!("s is {}", s);
    }
    foo(str);
    // 此处：str的所有权已经被转移
    // println!(" use str after call foo: {}", str);

    // 对于元组和数组：
    let arr = [3; 5];
    let arr1 = arr;
    println!("{:?}", arr);

    let arr_1 = ["11".to_string()];
    let arr_2 = arr_1;
    // 1、如果存储都是基础数据类型（实现copy trait）会自动复制,没有所有权问题
    // println!("{:?}", arr_1);

    let arr = (1);
    let arr1 = arr;
    println!("{:?}", arr);

    let arr = (1, (1, 2));
    let arr1 = arr;
    println!("{:?}", arr);

    let arr_1 = ("11".to_string(), 1);
    let arr_2 = arr_1;
    // 2、如果存储的是非基础数据类型(没有实现copy trait)会存在所有权转移问题
    // println!("{:?}", arr_1);

    // 数据必须在其所有引用销毁之前存活
    let v1 = vec![1, 2, 3];
    // drop函数会回收内存，后续无法使用
    drop(v1);
    println!("{}", v1[0]);
}

fn bar() -> &String {
    // 局部变量info在bar函数运行结束后被回收，不能返回引用
    let info = String::from("2333");
    &info
}
// 直接返回变量可以
fn foo() -> String {
    let info = String::from("2333");
    info
}
