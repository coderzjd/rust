fn main() {
    // Never Type
    // 1、表示永不返回的类型
    // 2、!可以转化为任意值
    
    // 1、有一个守护函数，一旦运行不会停止
    fn never_stop() -> ! {
        // loop {}函数的返回值就是never
        loop {}
    }

    let r1 = Some(1);
    let info = match r1 {
        Some(x) =>x,
        // panic!宏的返回值就是![Never Type],可以转化为任意值
        None => panic!("matc error!")
    };
}
