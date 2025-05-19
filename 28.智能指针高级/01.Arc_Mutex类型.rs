use std::sync::{Arc, Mutex};

fn main() {
    let v1: Vec<i32> = vec![];
    let a = Arc::new(Mutex::new(v1));
    a.lock().unwrap().push(2);
    println!("a is {:?}", a.lock().unwrap());
    // 1、原始值不是mut也可以修改
    // 因为Arc<Mutex<T>>组合起来就实现了线程安全的共享可变状、不违反Rust的借用规则，此时 v1 是不是mut已经不重要了

    // 2、如果lock赋值，再想修改就需要声明为mut
    // 因为这个赋值变量b有自己的借用规则
    let mut b = a.lock().unwrap();
    b.push(3);
    println!("b is {:?}", b);
    drop(b);
    // 3、n+1解锁时必须保证第n第解锁被释放，否则导致线程自己阻塞自己
    let mut c = a.lock().unwrap();
    c.push(3);
    println!("b is {:?}", c);
}
