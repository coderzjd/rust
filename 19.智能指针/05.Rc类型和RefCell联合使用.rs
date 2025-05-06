use std::{cell::RefCell, rc::Rc};
// Rc<T>与RefCell<T>联合使用
fn main() {
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let value = Rc::new(RefCell::new(1));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("a is {a:?}");
    println!("b is {b:?}");
    println!("c is {c:?}");


}
