fn main() {
    let mut num = 0;
    let info = 'aaaa: loop {
        println!("num is {num}");
        if num >= 5 {
            break "外层loopEnd";
        }
        num += 1;
        let mut num2 = 0;
        loop {
            println!("num2 is {num2}");
            if num2 >= 5 {
                break;
            }
            num2 += 1;
            if num >= 3 {
                break 'aaaa "内层loopEnd";
            }
        }
    };
    println!("info is {info}")
}
