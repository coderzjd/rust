fn main() {
    let v1 = vec![1, 2, 3];
    // 1、在运行时数组下标越界会触发panic!
    // let info = &v1[100];
    // println!("info is {}", info)
    // 2、手动触发
    panic!("主动触发panic!")
}
