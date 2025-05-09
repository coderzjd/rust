use std::thread;

fn main() {
    // Sync and Send Traits
    // Send 特征指示实施 Send 类型的值的所有权可以在线程之间转移。几乎所有 Rust 类型都是 Send
    // 完全由 Send 类型组成的任何类型也会自动标记为 Send。

    // Sync 表示它对于实现该trait的类型可以安全的从多个线程引用。原始类型是 Sync
    // 和完全由 Sync 类型组成的类型也是 Sync。

    const NUM: i32 = 998;
    let num_ref = &NUM;
    let num_ref_thread = num_ref;
    // num_ref_thread可以从主线程传递到子线程，所以： num_ref_thread(&i32) 是 Send的
    let handler = thread::spawn(move || {
        // 在子线程内num_ref_thread可以引用到NUM，所以：NUM(i32)是 sync 的
        println!("num_ref_thread is {num_ref_thread}")
    });
}
