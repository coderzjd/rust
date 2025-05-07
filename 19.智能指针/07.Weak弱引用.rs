use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

fn main() {
    // 防止引用循环： 使用Weak<T>代替Rc<T>
    // 1、Rc::clone()会创建强引用，只有强引用数量为0,Rc<T>指向的值才会被清理
    // 2、通过Rc::downgrade创建弱引用，不代表所有权、不影响垃圾回收，不会导致循环引用
    // 3、Weak<T>不能直接使用值，通过upgrade方法检查弱引用值是否存在
    // 存在：返回Some(Rc<T>)
    // 不存在返回None
    #[derive(Debug)]
    struct Node {
        value: i32,
        childern: RefCell<Vec<Rc<Node>>>,
        parent: RefCell<Weak<Node>>,
    }
    // 创建叶子节点
    let leaf = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        childern: RefCell::new(vec![]),
    });
    println!("获取根节点 {:?}", leaf.parent.borrow().upgrade());
    // 创建树枝节点，将叶子节点作为子节点
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        childern: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("弱引用数量： {}", Rc::weak_count(&branch));
    println!("获取根节点 {:?}", leaf.parent.borrow().upgrade())
}
