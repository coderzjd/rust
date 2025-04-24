//可变引用和不可变引用的关系，借用检查器会将runtime提前到compiler
fn main() {
    // 1、在heap上开辟空间存储[1,2,3]
    let mut v = vec![1, 2, 3];
    // 使用num指向v中的数据
    let num = &v[2];
    // 修改heap上v的数据： 1、新建一片heap空间存储[1,2,3,4]，原始v指向的地址失效了
    // 因为num还指向原始v的地址，导致出错
    // cannot borrow `v` as mutable because it is also borrowed as immutable mutable borrow occurs here
    // v.push(4);

    // 是因为v是可变引用，num是不可变可变引用
    println!("num is {}", num);

    // 但是修改可变引用和不可变可变引用的顺序就正常了
    let mut v1 = vec![1, 2, 3];
    let num1 = &v1[2];
    println!("num is {}", num);
    // 在使用可变引用之前，不可变引用已经失效了；所以代码可以运行
    v.push(4);
}
