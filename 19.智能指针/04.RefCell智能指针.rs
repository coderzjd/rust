use std::cell::RefCell;

fn main() {
    // RefCell<T>
    // 当你确认代码遵守了借用规则，但是编译器无法理解（无法编译通过）
    // 1、rust编译器是保守的
    // RefCell<T>只适用于单线程，多线程使用会在编译时报错

    // 与Box<T>的区别
    // 1、Box<T>出现编译错误无法编译通过
    // 2、RefCell<T>在运行时执行，出现错误触发panic

    // 对于RefCell<T>
    // 1、使用borrow_mut方法返回值是RefMut<T>(相当于一个可变引用)可以通过解引用修改值
    // 2、使用borrow方法返回值是Ref<T>(相当于一个不可变引用)，通过解引用获取值
    struct Counter {
        count: RefCell<i32>,
    }
    impl Counter {
        fn new() -> Self {
            Counter {
                count: RefCell::new(0),
            }
        }
        fn increment(&self) {
            // 获取可变引用，这里会进行运行时的借用检查，编译时不检查
            let mut num = self.count.borrow_mut();
            // 运行时违反借用规则还是会报错、触发panic
            // let mut num2 = self.count.borrow_mut();
            // *num2 += 1;
            *num += 1;
        }
        fn get_count(&self) -> i32 {
            // 获取不可变引用
            let num = self.count.borrow();
            *num
        }
    }
    let counter = Counter::new();
    counter.increment();
    println!("Count: {}", counter.get_count()); // 输出：Count: 1
    counter.increment();
    println!("Count: {}", counter.get_count()); // 输出：Count: 2
}
