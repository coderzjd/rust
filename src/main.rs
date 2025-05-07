fn main() {
    // Sync and Send Traits
    // Send marker 特征指示实施 Send 类型的值的所有权可以在线程之间转移。几乎所有 Rust 类型都是 Send
    // 完全由 Send 类型组成的任何类型也会自动标记为 Send。

    // Sync marker trait 表示它对于实现 sync 以从多个线程引用。换句话说，任何类型 T 都是 Sync if &T （对 T 的不可变引用） 为 Send，
    // 这意味着该引用可以安全地发送到另一个线程。与 Send 类似，原始类型是 Sync 和完全由 Sync 类型组成的类型也是 Sync。
}
