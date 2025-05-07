use std::{cell::RefCell, rc::Rc};

fn main() {
    // 内存泄露
    // rust的安全保障使得意外的内存泄露很难发生，也不是不可能
    // 完全防止内存泄露不是rust 的保证，因为内存泄露是内存安全的
    // 通过Rc<T>和 RefCell<T>可以创建出循环引用，导致内存泄露
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }
    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }
    use List::{Cons, Nil};
    // 创建变量a
    let a = Rc::new(Cons(1, RefCell::new(Rc::new(Nil))));
    // 创建变量b,且指向a
    let b = Rc::new(Cons(2, RefCell::new(Rc::clone(&a))));
    if let Some(x) = a.tail() {
        // 在通过RefCell类型让a指向b，构成循环引用
        *x.borrow_mut() = Rc::clone(&b)
    }
    // thread 'main' has overflowed its stack
    // println!("a.tail {:?}", a.tail())
}
