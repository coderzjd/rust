fn main() {
    let mut num = 0;
    // loop循环相当于while true
    loop {
        num += 1;
        if num > 5 {
            break;
        }
        if num == 2 {
            continue;
        }
        println!(" num is {num}");
    }
}
