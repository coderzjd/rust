fn main() {
    let mut num = 0;
    let info = loop {
        println!("num is {num}");
        num += 1;
        if num >= 5 {
            break "loopEnd";
        }
    };
    println!("info is {info}")
}
