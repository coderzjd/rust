use std::sync::Mutex;

fn main() {
    // Mutex互斥锁
    // 1、互斥锁在任何给定时间只允许一个线程访问某些数据
    // 2、要访问互斥锁中的数据，线程必须请求获取互斥锁的锁
    // 3、锁和互斥锁是一种数据结构、用于跟踪谁当前对数据的独占访问权
    // 4、互斥锁被被描述为通过锁定系统来保护它持有的数据

    // 使用规则
    // 1、在使用数据之前，必须尝试获取锁。
    // 2、当您完成互斥锁保护的数据后，您必须解锁数据，以便其他线程可以获取锁。

    let m = Mutex::new(5);
    {
        // 获取锁，返回一个 `MutexGuard`
        let mut num = m.lock().unwrap();
        println!("num is {}", num);
        // 此处MutexGuard相当于智能指针，解引用
        *num += 1;
        // 走出当前作用域num会自动解锁
    }
    let mut num = m.lock().unwrap();
    *num += 1;
    // 手动解锁
    drop(num);
    println!("m is {m:?}")
}
