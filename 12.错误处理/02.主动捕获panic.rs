use std::panic;
fn main() {
    let arr = vec![1, 2, 3];
    // 类似于trycatch
    let panic_info = panic::catch_unwind(|| {
        let info = &arr[100];
        info
    });
    match panic_info {
        Ok(item) => {
            println!("item is {item}")
        }
        Err(e) => {
            println!("err is {e:?}")
        }
    }
}
