use std::rc::Rc;

fn main() {
    // 引用计数智能指针
    // Rc<T>
    // 1、有些情况下一个值可能被多个所有者拥有【图数据结构】
    // 2、Rc<T>可以开启多重所有权
    //      1、跟踪一个值引用的数量、可以判断该值是否还在使用
    //      2、如果没有引用、就可以清理了
    // 适用场景：
    // 1、在heap上分配数据，供程序的多个部分读取、但在编译时无法确认哪个部分会最后完成对数据的使用
    // 2、只可以用在单线程场景

    // Rc基本使用(此处s2和s3都指向s1)
    let s1 = Rc::new(String::from("233"));
    println!("s1 的引用数量为 {}", Rc::strong_count(&s1));
    let s2 = Rc::clone(&s1);
    println!("s1 的引用数量为 {}", Rc::strong_count(&s1));
    {
        let s3 = Rc::clone(&s1);
        println!("s1 的引用数量为 {}", Rc::strong_count(&s1));
        println!("s3 is {s3}");
    }
    println!("s3 is {s2}");
    println!("s1 的引用数量为 {}", Rc::strong_count(&s1));

    // 使用Rc保存简单图结构
    enum ListRc {
        ConsRc(i32, Rc<ListRc>),
        Null,
    }
    use ListRc::{ConsRc, Null};
    let rc1 = Rc::new(ConsRc(1, Rc::new(Null)));
    let rc2 = ConsRc(1, Rc::clone(&rc1));
    let rc3 = ConsRc(1, Rc::clone(&rc1));
}
